---
name: emergence-auditor
description: Audit engine and simulation codebase to guarantee zero-scripting emergent complexity, eliminate hardcoded teleological scenarios, verify orthogonal affordances, enforce double-entry atomic ledger conservation, and ensure 100% bit-exact determinism.
---

# 🌌 Emergence-Auditor Skill: Code Review & Anti-Pattern Diagnostic

Skill ini digunakan untuk mengaudit seluruh kode, modul baru, sistem interaksi, dan PR di repositori `economy` guna menjamin **Prinsip Kemunculan Murni (*Pure Emergence*)** dan mengeliminasi segala bentuk **Skenario Buatan (*Hardcoded Teleology*)**.

---

## 🔍 Checklist Audit 9 Prinsip Emas

Saat meninjau berkas atau merancang modul baru di `src/`, jalankan 9 pemeriksaan wajib ini:

### 1. Uji Anti-Teleologi (The Teleological Trap Check)
- [ ] **Pertanyaan**: Apakah ada logika berbasis *timeline*, tick target, atau event trigger buatan?
- [ ] **Red Flags**:
  - `if current_tick == 3650 { trigger_industrial_revolution(); }`
  - `if agent.hungry { agent.go_to_market_and_buy(); }`
  - `if agent.age > 20 { agent.become_merchant(); }`
- [ ] **Solusi Emergent**: Ganti dengan hukum mikro invarian. Agen hanya merespons kebutuhan metabolik lokal (rasa lapar, kelelahan, risiko mati). Spesialisasi kerja dan revolusi teknologi harus lahir dari keunggulan marjinal yang ditemukan agen secara mandiri.

### 2. Uji Ortogonalitas & Afordansi (Square Mechanics Check)
- [ ] **Pertanyaan**: Apakah sistem A dikawinkan secara khusus (*tightly coupled*) ke sistem B ($O(N^2)$ dependencies)?
- [ ] **Red Flags**:
  - `axe.chop_oak_tree(&mut oak_tree)`
  - `agent.use_stone_axe_on_specific_timber_node(...)`
- [ ] **Solusi Emergent**: Ekstrak sifat (*tags/affordances* umum).
  - Objek: `Flammable`, `Woody`, `Solid`, `Edible`.
  - Alat: `CuttingEdge(Sharpness)`, `TensileStrength`.
  - Interaksi `CuttingEdge` + `Woody` menghasilkan serpihan kayu tanpa peduli apakah itu pohon ek, tiang rakit, atau pagar rumah.

### 3. Uji Kekekalan Materi & Energi (Zero Ex-Nihilo Check)
- [ ] **Pertanyaan**: Apakah ada item, energi, atau uang yang tercipta atau terhapus dari ketiadaan?
- [ ] **Red Flags**:
  - `agent.inventory.insert(ItemId::STONE_AXE, 1)` tanpa mengurangi bahan mentah.
  - `agent.wallet += 100.0` sebagai hadiah naik level atau quest.
  - Menghapus item saat dikonsumsi tanpa menghasilkan energi atau limbah.
- [ ] **Solusi Emergent**:
  - Setiap output mensyaratkan input materi setara + kalori tenaga kerja.
  - Setiap perubahan kepemilikan dan transformasi fisik wajib dicatat atomik di `LedgerEntry` dengan `trx_id` valid.

### 4. Uji Rasionalitas Terbatas & Informasi Lokal (Bounded Local Rationality)
- [ ] **Pertanyaan**: Apakah agen membaca database global atau dipandu koordinator tak terlihat?
- [ ] **Red Flags**:
  - Agen membaca harga termurah di seluruh dunia dari `MarketRegistry::get_lowest_price()`.
  - Agen langsung mengetahui posisi sumber daya di seberang benua tanpa pernah menjelajah.
- [ ] **Solusi Emergent**:
  - Agen hanya membaca state lokal: inventori sendiri, memori interaksi masa lalu, dan sel spasial dalam radius pandang ($r \le \text{view\_range}$).
  - Harga pasar adalah residu statistik dari barter bilateral bilateral lokal, bukan angka yang dihitung rumus pusat.

### 5. Uji Koordinasi Lingkungan (Stigmergy Check)
- [ ] **Pertanyaan**: Apakah agen saling mengontrol secara langsung (*remote invocation*)?
- [ ] **Red Flags**: `agent_a.order_agent_b_to_trade(item)`.
- [ ] **Solusi Emergent**: Agen berinteraksi melalui **modifikasi spasial/lingkungan**:
  - Agen menebang pohon $\to$ stok hutan berkurang, kematangan biologis turun $\to$ agen lain melihat hutan gundul dan berpindah tempat.
  - Agen menimbun barang di suatu sel $\to$ sel tersebut menjadi sentra akumulasi $\to$ menarik pedagang lain (lahirnya pasar secara fisik).

### 6. Uji Umpan Balik Non-Linear (Feedback Loop Balancing)
- [ ] **Pertanyaan**: Apakah stabilitas ekosistem dijaga oleh batasan kaku buatan (*hardcoded caps*)?
- [ ] **Red Flags**: `if agent_population >= 100 { stop_breeding(); }`.
- [ ] **Solusi Emergent**: Seimbangkan loop positif dan negatif alami:
  - *Loop Positif*: Energi melimpah $\to$ produksi alat modal $\to$ panen 3x lipat $\to$ akumulasi aset.
  - *Loop Negatif*: Over-eksploitasi $\to$ biomassa $< 0.40$ $\to$ penalti panen anjlok 75% (*Tragedy of the Commons*) $\to$ kelaparan $\to$ populasi turun atau beralih mencari daerah baru.

### 7. Uji Afordansi Cerdas (Smart Objects & Smart Terrain)
- [ ] **Pertanyaan**: Apakah pohon keputusan AI agen berisi ratusan percabangan `if-else` untuk setiap benda dunia?
- [ ] **Red Flags**: `match object_type { Oak => ..., Pine => ..., Rock => ..., Stream => ... }`.
- [ ] **Solusi Emergent**: Objek yang mengiklankan kapabilitasnya:
  - Laut Dalam mengiklankan: `RequiresVessel(true), EnergyCost(High)`.
  - Sumber Pangan mengiklankan: `Yields(Food), Calories(500), Renewal(Medium)`.
  - AI agen hanya memegang dorongan biologis generik (kurangi lapar, kurangi lelah, maksimalkan keamanan).

### 8. Uji Dualitas Ontologi Materi vs Informasi
- [ ] **Pertanyaan**: Apakah ilmu pengetahuan disamakan dengan benda fisik?
- [ ] **Red Flags**: Saat agen mengajari agen lain, ilmu pengetahuan terhapus dari pengajar (`agent_a.remove_item(knowledge)`).
- [ ] **Solusi Emergent**:
  - Materi fisik bersifat **Rival** (berpindah tangan $\implies$ pemilik lama kehilangan).
  - Gagasan bersifat **Non-Rival** (diajarkan ke murid $\implies$ pengajar tetap memiliki ilmu tersebut). Jasa pendidikan lahir secara organik saat penemu menukar transfer ilmu non-rival dengan paket pangan fisik rival.

### 9. Uji Determinisme Radikal (100% Bit-Exact Verification)
- [ ] **Pertanyaan**: Apakah ada sumber non-determinisme tersembunyi?
- [ ] **Red Flags**:
  - Penggunaan `std::collections::HashMap` standar (SipHash non-deterministik).
  - Panggilan `rand::thread_rng()` atau `SystemTime::now()` di dalam loop logika simulasi.
  - Pengurutan floating-point tanpa penanganan `NaN` deterministik.
- [ ] **Solusi Emergent**:
  - Gunakan `BTreeMap` atau `IndexMap`.
  - Seluruh bilangan acak berakar dari `ChaCha8Rng` yang diinjeksi via `RngPort`.
  - `cargo test` wajib memverifikasi bahwa dua simulasi dengan seed yang sama menghasilkan bit-exact data yang identik.

### 10. Uji Kemunculan Spontan Institusi Ekonomi (Emergence of Currency, Banking & Firm)
- [ ] **Pertanyaan**: Apa yang menghambat atau mencegah munculnya mata uang, bank/kredit, dan perusahaan secara murni emergent?
- [ ] **Red Flags**:
  - *Hambatan Mata Uang (Barter Trap)*: Agen hanya mau bertukar jika terjadi *double coincidence of wants* langsung. Agen menolak menerima barang berlikuiditas tinggi (garam, kerang, gandum) yang tidak langsung mereka konsumsi sendiri, sehingga *indirect exchange* (Carl Menger) tidak pernah lahir.
  - *Hambatan Perbankan & Kredit (Zero-Storage Lending Trap)*: Agen kaya yang memiliki surplus pangan di lumbung/tempayan membiarkan barangnya menganggur, sementara agen miskin di musim paceklik kelaparan karena tidak ada mekanisme pinjam-meminjam dengan janji bayar di masa panen (*credit ledger / promissory IOUs*).
  - *Hambatan Perusahaan / Firma (Autarkic Enterprise Trap)*: Setiap agen bekerja sendiri-sendiri (*pure individual autarky*). Tidak ada kontrak kerja sama (Ronald Coase, *Theory of the Firm*) di mana pemilik alat modal (misal: pemilik batu gilang atau rakit) mempekerjakan agen lain dan membagi hasil produksi secara proporsional.
- [ ] **Solusi Emergent**:
  - *Mata Uang Mengerian*: Agen mengevaluasi daya jual pasar (*saleability / Absatzfähigkeit*) komoditas: jika komoditas $C$ (misal `COWRIE_SHELLS`, `SALT`, `GRAIN`) memiliki rasio likuiditas dan keawetan jauh lebih tinggi daripada barang yang dipegang, agen rasional menerima $C$ sebagai perantara tukar (*medium of exchange*).
  - *Lumbung Kredit & Perbankan Purba*: Agen dapat menitipkan surplus pangan ke tempayan/lumbung bersama dan memperoleh unit kredit/hak klaim yang dapat dipinjamkan dengan bunga wajar bahan.
  - *Proto-Firma / Koalisi Produksi Coasean*: Dua atau lebih agen dapat membentuk aliansi kerja sama: pemilik alat modal menyediakan katalis, pekerja menyediakan tenaga kerja, dan hasil panen/manufaktur dibagi bersama.

---

## 🛠️ Perintah Eksekusi Audit Otomatis

Gunakan bash/grep di terminal untuk memindai pelanggaran:

```bash
# 1. Deteksi iterasi HashMap tak deterministik di core domain
grep -rn "HashMap" src/core/domain

# 2. Deteksi pemanggilan PRNG thread-local liar
grep -rn "thread_rng" src/

# 3. Deteksi penambahan/pengurangan item tanpa melalui Ledger
grep -rn "\.inventory\." src/core/systems/

# 4. Deteksi hardcoded caps buatan
grep -rn "MAX_AGENTS" src/core/

# 5. Jalankan verifikasi determinisme
cargo test --test determinism_test
```
