# 🌌 Golden Architectural Directives: Emergence over Hardcoding

Dokumen ini adalah **panduan absolut dan aturan sistem (custom rulebook)** yang wajib dipatuhi dalam pengembangan codebase simulator ekonomi ini. Tujuannya adalah **memaksimalkan kemunculan fenomena makro yang spontan (*pure emergence*)** dan **menghilangkan segala bentuk skenario buatan (*teleological hardcoded scripting*)**.

---

## 🏛️ 1. Filosofi Dasar: Complex Adaptive Systems (CAS)

Dalam ekonomi neoklasik dan game konvensional, desainer sering mendikte hasil akhir secara terencana:
- Memberikan teknologi baru pada tahun tertentu.
- Menetapkan harga pasar dari rumus kurva penawaran-permintaan agregat global.
- Memberikan uang fiat buatan (*wallet balance*) kepada setiap agen.
- Menyematkan flag kemampuan boolean (misal `has_boat = true` atau `mining_skill_level = 5`).

**Pendekatan tersebut DILARANG di repositori ini.**

Sebagai gantinya, kita menerapkan prinsip **Complex Adaptive Systems (CAS)**, terinspirasi oleh teori ekonomi kompleksitas (W. Brian Arthur / Santa Fe Institute), simulasi ekologis (*Dwarf Fortress*, *Breath of the Wild Chemistry Engine*), dan pemodelan berbasis agen (*Agent-Based Computational Economics*):
> *"Jangan pernah memprogram hasil makro (*outcomes*). Programlah aturan mikro (*foundational constraints*), sifat materi (*affordances*), dan hukum kekekalan (*conservation laws*), lalu biarkan fenomena makro lahir dan mengorganisasi dirinya sendiri secara spontan."*

---

## ⚡ 2. Sembilan Prinsip Pemrograman untuk Emergence (*The 9 Emergent Laws*)

### Prinsip 1: Hukum Mikro Invarian (Aturan Dasar Menggantikan Skenario)
- **Anti-Pattern**: Menulis skrip berbasis waktu: `if tick == 3650 { trigger_industrial_revolution(); }` atau `if agent.hungry { go_to_market_and_buy(); }`.
- **Emergent Pattern**: Rancang hukum alam yang kaku dan konsisten (metabolisme basal, peluruhan energi, termodinamika). Agen hanya berusaha mempertahankan kelangsungan hidupnya dari ancaman entropi biologis. Revolusi industri atau perdagangan pasar akan muncul hanya jika agen secara individu menemukan bahwa spesialisasi dan akumulasi modal lebih menguntungkan kelangsungan hidup mereka daripada bertahan hidup sendiri.

### Prinsip 2: Mekanika Kotak & Saling Tegak Lurus (*Square & Orthogonal Mechanics*)
- **Anti-Pattern**: Mengawinkan sistem A secara spesifik ke sistem B (*point-to-point hardcoding*), misalnya `axe.chop(oak_tree)`. Jika ada $N$ alat dan $M$ objek, ini memicu ledakan kombinatorik $O(N \times M)$ baris kode khusus.
- **Emergent Pattern**: Gunakan konsep sifat dan afordansi umum (*tags & affordances*):
  - Entitas lingkungan memiliki sifat: `Woody`, `Fibrous`, `Flammable`, `Solid`.
  - Perkakas memiliki kemampuan: `CuttingEdge(Sharpness)`, `TensileStrength`.
  - Pertemuan antara `CuttingEdge` dan bahan `Woody` secara otomatis menghasilkan serpihan kayu tanpa peduli apakah objeknya adalah pohon ek, tiang rakit, atau pagar rumah. $N$ sistem yang saling ortogonal akan menghasilkan interaksi eksponensial secara otomatis.

### Prinsip 3: Hukum Kekekalan Mutlak & Siklus Tertutup (*Zero Ex-Nihilo*)
- **Anti-Pattern**: Menciptakan uang, makanan, atau barang dari ketiadaan (*spawn out of thin air*) saat dibutuhkan, atau menghapus barang begitu saja saat dikonsumsi tanpa jejak.
- **Emergent Pattern**:
  - **Kekekalan Massa & Energi**: Sebuah kapak batu tidak boleh tiba-tiba ada; ia harus mengonsumsi 5 unit kayu mentah dan kalori tenaga kerja dari agen yang merakitnya.
  - **Siklus Tertutup (*Closed Thermodynamic Loop*)**: Nutrisi yang dimakan agen dikonversi menjadi energi kerja atau kotoran/limbah yang menyuburkan tanah.
  - **Double-Entry Ultimate Ledger**: Setiap perubahan kepemilikan, transformasi manufaktur, atau panen wajib tercatat dalam transaksi atomik ber-`trx_id`. Tidak ada atom atau nilai ekonomi yang boleh bocor di luar buku besar.

### Prinsip 4: Desentralisasi Otonom & Rasionalitas Terbatas (*Bounded Local Rationality*)
- **Anti-Pattern**: Agen menanyakan harga termurah ke "database pasar global" atau dipandu oleh "Invisible Hand Coordinator" pusat yang mengatur alokasi sumber daya secara serentak.
- **Emergent Pattern**:
  - Agen hanya memiliki **informasi lokal**: apa yang ada di inventori pribadinya, apa yang ada di koordinat pandangannya ($r \le \text{jarak pandang}$), dan apa yang diingat dari interaksi masa lalu.
  - Penilaian nilai dilakukan secara **subjektif**:
    $$U(\text{Item}, \text{CalorieReserve}, \text{CurrentStock}) = \frac{\text{BaseUtility} \times \text{Urgency}}{1.0 + \text{Stock}}$$
  - Harga pasar agregat (*clearing price*) tidak pernah dihitung oleh satu rumus pusat; harga adalah residu statistik dari ribuan kesepakatan barter bilateral lokal antarindividu.

### Prinsip 5: Koordinasi Tak Langsung melalui Lingkungan (*Stigmergy*)
- **Anti-Pattern**: Agen saling mengirim sinyal koordinasi global atau memanggil method agen lain secara paksa (`agent_b.come_here()`).
- **Emergent Pattern**: Gunakan konsep **Stigmergy** (seperti semut yang meninggalkan feromon):
  - Agen berinteraksi dengan **mengubah lingkungan spasial**.
  - Agen menebang hutan $\to$ stok hutan berkurang, kematangan biologis anjlok $\to$ agen lain yang lewat melihat hutan gundul dan memutuskan mencari daerah baru.
  - Agen menimbun barang di suatu lokasi $\to$ lokasi tersebut menjadi sentra penumpukan materi $\to$ agen lain tertarik datang karena peluang barter tinggi $\to$ lahirnya pasar/kota secara fisik tanpa ada skrip `found_city()`.

### Prinsip 6: Umpan Balik Non-Linear (Keseimbangan Positif & Negatif)
- **Positive Feedback Loop (Penguatan / Diferensiasi)**:
  - Agen berenergi tinggi $\to$ waktu luang banyak $\to$ menemukan resep perkakas $\to$ merakit kapak $\to$ panen 3x lipat $\to$ mengakumulasi kekayaan dan modal. Fenomena konsentrasi modal lahir secara organik.
- **Negative Feedback Loop (Penyeimbang / Stabilisasi Alami)**:
  - Eksploitasi berlebihan $\to$ kematangan biologis anjlok di bawah 0.40 $\to$ penalti kualitas hasil panen hingga tersisa 25% (*Tragedy of the Commons*) $\to$ agen kelaparan atau terpaksa menghentikan penebangan $\to$ hutan mendapat waktu untuk regenerasi logistik.
- Keseimbangan ekosistem tercipta dari gesekan dua loop ini, bukan dari *hardcoded cap* atau *soft limit* buatan.

### Prinsip 7: Afordansi Cerdas (*Smart Objects & Smart Terrain*)
- **Anti-Pattern**: Pohon keputusan AI agen yang memiliki ribuan baris `if-else` untuk setiap kemungkinan benda di dunia.
- **Emergent Pattern**:
  - Letakkan kecerdasan pada objek/medan (*Affordance-Based Design*). Objek "mengiklankan" apa yang bisa dilakukannya:
    - Laut Dalam mengiklankan: `RequiresVessel(true), EnergyCost(High)`.
    - Sumber Ikan mengiklankan: `Yields(Fish), Nutrition(500 kcal), Pace(Medium)`.
  - Agen hanya memiliki dorongan motif umum: meminimalkan rasa lapar, memaksimalkan kenyamanan/keamanan, mengurangi kelelahan. Agen mencocokkan dorongan motifnya dengan afordansi objek di sekitarnya.

### Prinsip 8: Dualitas Ontologi Materi vs Informasi
- **Materi Fisik Bersifat Rival (*Rival Physical Goods*)**:
  - Gandum, kayu, batu, rakit. Jika agen A memberikan barang fisik ke agen B, agen A **kehilangan** barang tersebut. Jumlah materi selalu konservatif.
- **Gagasan & Pengetahuan Bersifat Non-Rival (*Non-Rival Ideas*)**:
  - Resep rakit, cetak biru kapak, teknik pengawetan garam. Jika guru A mengajarkan resep kepada murid B, guru A **tidak kehilangan** ilmu tersebut.
  - Penemuan muncul dari waktu luang (*leisure & surplus calories*).
  - Jasa pendidikan (*apprenticeship*) muncul ketika pemilik ilmu menjual transfer cetak biru non-rival dengan imbalan paket pangan fisik rival. Sektor jasa lahir secara organik tanpa perlu dibuat modul khusus `class SchoolSystem`.

### Prinsip 9: Observabilitas Radikal & Determinisme Bit-Exact
- Karena sistem *emergent* sangat rentan terhadap efek domino tak terduga (*butterfly effect*), sistem ini menuntut:
  1. **Determinisme 100%**: Tidak boleh ada *thread race conditions*, iterasi `HashMap` tak berurut, atau pemanggilan PRNG non-deterministik. Semuanya berakar dari satu seed master via `ChaCha8Rng` dan iterasi `BTreeMap`.
  2. **Auditabilitas Total**: Setiap peristiwa ekonomi, penemuan ide, atau transfer materi dicatat ke `LedgerEntry` dan diekspor ke Apache Parquet.
  3. **Penolakan Debugging Spekulatif**: Jika terjadi ketidakseimbangan simulasi, ubah *levers* kendala fisik dasar (laju regenerasi biologis, kebutuhan kalori dasar), BUKAN menambal perilaku agen dengan `if` khusus.

---

## 🚫 3. Daftar Hitam Anti-Pattern (*Code Smells to Avoid*)

| Jangan Lakukan (*Anti-Pattern*) | Solusi Emergent Sejati (*Best Practice*) |
| :--- | :--- |
| **The Teleological Trap**: Membuat fungsi `evolve_to_bronze_age()` atau `spawn_market()`. | Biarkan agen mengumpulkan tembaga dan timah sendiri karena manfaat utilitas alat logam mengalahkan batu. |
| **The Omniscient Coordinator**: Membuat struct `Market` yang mengumpulkan seluruh penawaran dan permintaan lalu menentukan satu harga ekuilibrium. | Biarkan barter bilateral terjadi antara dua agen yang berpapasan secara spasial berdasarkan utilitas marjinal subjektif masing-masing. |
| **Magic Currency**: Memberikan agen properti `wallet: f64` atau `gold_coins: 100`. | Hilangkan variabel dompet. Uang adalah komoditas fisik biasa di inventori yang memiliki likuiditas dan ketahanan simpan tertinggi (*Carl Menger*). |
| **God-Mode Buffs**: Memberikan status `agent.mining_speed += 0.5` setelah naik level. | Agen harus mengorbankan waktu dan bahan mentah untuk memproduksi perkakas fisik (`STONE_AXE`) yang memberikan keunggulan mekanis nyata. |
| **Instant Teleportation**: Agen langsung berpindah ke lokasi sumber daya saat lapar. | Agen harus berjalan langkah demi langkah melintasi grid spasial 2D, membakar kalori perjalanan sesuai friksi medan dan hambatan perairan. |

---

## 🛠️ 4. Panduan Implementasi Fitur Baru di Repositori Ini

Setiap kali Anda hendak menambahkan fitur, item, atau mekanika baru ke dalam repositori `economy`, ajukan 4 pertanyaan filter ini:

1. **Apakah ini hukum alam universal atau skenario yang diatur?**
   - Jika ini skenario yang diatur $\implies$ **TOLAK**. Rancang ulang menjadi hukum alam mikro.
2. **Apakah interaksi ini menambah dependensi khusus antara dua modul konkret?**
   - Jika ya $\implies$ Ekstrak menjadi *trait/affordance* umum agar objek lain dapat memanfaatkan interaksi yang sama tanpa modifikasi kode.
3. **Apakah hukum kekekalan materi dan buku besar dihormati?**
   - Pastikan setiap input materi dan output materi memiliki catatan ganda (*double-entry*) di Ultimate Ledger dengan `trx_id` yang valid.
4. **Apakah pengujian determinisme tetap terjaga?**
   - Jalankan `cargo test`. Dua simulasi dengan seed yang sama wajib menghasilkan bit-exact data yang identik.

---

*Dengan memegang teguh prinsip-prinsip ini, simulator ekonomi ini tidak sekadar menjalankan animasi angka, melainkan menjadi laboratorium hidup bagi evolusi peradaban manusia yang otentik dan murni.*
