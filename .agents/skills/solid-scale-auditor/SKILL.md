---
name: solid-scale-auditor
description: Audit codebase files for excessive length, Single Responsibility Principle (SRP) violations, God-objects, and architectural entanglement. Forecasts scaling bottlenecks and provides concrete SOLID decomposition patterns to split bloated files cleanly before future complexity explodes.
---

# 📐 SOLID Scale-Auditor Skill: Architecture, Modular Decomposition & Scale Forecasting

Skill ini digunakan untuk mengaudit seluruh berkas dalam repositori `economy` guna mendeteksi **berkas yang barisnya terlalu besar (*code bloat*)**, mendiagnosis **pelanggaran prinsip SOLID (terutama Single Responsibility Principle & Interface Segregation)**, serta **memprediksi titik jenuh skalabilitas (*future scale bottlenecks*)** sebelum kompleksitas sistem meledak.

---

## 🧭 Mengapa Ukuran Berkas Berkorelasi Erat dengan Pelanggaran SOLID?

Dalam rekayasa perangkat lunak modern (Robert C. Martin / Uncle Bob), berkas yang membengkak di atas **300–500 baris kode (LOC)** hampir selalu merupakan indikasi kuat terbentuknya **God Object / Swiss Army Knife Anti-Pattern**:
1. **Multiple Reasons to Change (Pelanggaran SRP)**: Jika berkas menangani kalkulasi matematika, manipulasi data inventori, transaksi ledger, dan format presentasi sekaligus, berkas tersebut memiliki 4 alasan berbeda untuk dimodifikasi.
2. **Merge Conflict Magnet**: Dalam kolaborasi multi-agen atau tim developer, berkas berukuran raksasa menjadi sumber konflik git tertinggi setiap kali ada fitur baru.
3. **Cognitive Overload & Fragility**: Pengujian unit (*unit test*) menjadi rumit karena membutuhkan ratusan baris mock/setup hanya untuk menguji satu fungsi kecil.

---

## 📊 Matriks Ambang Batas Baris Kode (Line-Count Thresholds)

Gunakan standar ambang batas ini saat mereview kode di `src/`:

| Status | Rentang LOC | Diagnosis Arsitektur | Tindakan Agen |
| :--- | :--- | :--- | :--- |
| 🟢 **Green** | **< 250 LOC** | **High Cohesion, Single Responsibility**. Berkas memiliki fokus sempit, mudah diuji secara modular, dan independen. | Pertahankan. |
| 🟡 **Yellow** | **250 - 450 LOC** | **Growth Strain / Warning Zone**. Berkas mulai menampung tanggung jawab sekunder atau cabang logika tambahan. | Pantau dan siapkan rencana ekstraksi modul. |
| 🔴 **Red** | **> 450 LOC** | **God Object / Critical SRP Violation**. Berkas menampung multi-konsep yang saling terikat erat (*tightly coupled*). | **Wajib Dipecah (*Decomposition Required*)**. |

---

## 🔍 Diagnosis 5 Prinsip SOLID terhadap Ukuran Berkas

### 1. S — Single Responsibility Principle (SRP)
- **Pertanyaan Audit**: Berapa banyak aktor atau pemangku kepentingan yang berkepentingan mengubah berkas ini?
- **Red Flags**:
  - `exchange_system.rs` (>500 LOC): Mengatur penemuan ide (*eureka*), pemanenan sumber daya alam, produksi alat modal, jasa pendidikan magang, dan barter komoditas fisik dalam satu fungsi raksasa.
  - `main.rs` (>350 LOC): Mengatur parsing CLI, resolusi commit git, seeding populasi, pembuatan terrain peta, registrasi 8 indikator statistik, loop simulasi, hingga perenderan tabel ASCII.
- **Solusi Skalabilitas**: Pecah ke dalam sub-handler atau domain pipeline terisolasi (misal: `HarvestPipeline`, `CraftingPipeline`, `BarterEngine`).

### 2. O — Open/Closed Principle (OCP)
- **Pertanyaan Audit**: Apakah penambahan 1 jenis komoditas, peran sosial, atau tabel statistik baru mewajibkan pengeditan berkas-berkas besar yang sudah ada?
- **Red Flags**:
  - Terdapat blok `match item_id.0 { 101 => ..., 102 => ..., 103 => ... }` raksasa yang diulang-ulang di berbagai modul.
- **Solusi Skalabilitas**: Gunakan **Registry Pattern** dan **Polymorphic Traits** (seperti `ItemRegistry::canonical()` dan trait `StatisticalTableCalculator`), sehingga penambahan fitur baru dilakukan via ekstensi tanpa mengutak-atik kode lama.

### 3. L — Liskov Substitution Principle (LSP)
- **Pertanyaan Audit**: Apakah ada sub-tipe atau implementor trait yang melempar `panic!()` atau membatalkan kontrak metode karena tidak relevan baginya?
- **Red Flags**: Objek non-fisik (seperti Jasa atau Gagasan) dipaksa mengimplementasikan metode fisik seperti `decay_in_warehouse()` yang mengembalikan error atau nilai nol palsu.
- **Solusi Skalabilitas**: Pisahkan sifat fisik dan non-fisik ke dalam trait terpisah.

### 4. I — Interface Segregation Principle (ISP)
- **Pertanyaan Audit**: Apakah trait memaksa implementor untuk menyediakan metode yang tidak pernah digunakan oleh konsumennya?
- **Red Flags**: Trait `AgentStorePort` yang menggabungkan pembacaan demografi, mutasi inventori, hubungan keluarga, dan memori masa lalu dalam 1 interface raksasa 40 metode.
- **Solusi Skalabilitas**: Pisahkan menjadi peran terfokus: `Identifiable`, `HasLifecycle`, `SocialActor`, `EconomicActor`.

### 5. D — Dependency Inversion Principle (DIP)
- **Pertanyaan Audit**: Apakah domain logic tingkat tinggi bergantung langsung pada modul tingkat rendah (I/O, Parquet, SystemTime, PRNG OS)?
- **Solusi Skalabilitas**: Core domain hanya berbicara melalui `Port` (Hexagonal Architecture).

---

## 🔮 Radar Prediksi Skalabilitas Masa Depan (Scaling Forecast Radar)

Ketika menilai berkas yang mendekati atau melampaui 400 LOC, agen **wajib memproyeksikan lintasan pertumbuhan (*scale trajectory*)** menggunakan 3 indikator:

```
Tingkat Risiko Skala = (LOC Saat Ini) × (Kecepatan Fitur Masa Depan) × (Faktor Keterkaitan / Coupling)
```

### Proyeksi Tren Skala pada Modul Kunci `economy`:

1. **`src/core/systems/exchange_system.rs` (Saat ini: ~540 LOC)**:
   - *Prediksi Skala*: Jika simulasi menambahkan sistem pajak, lembaga perbankan, uang fiat/kredit, dan kontrak sewa tanah, berkas ini akan meledak hingga **> 2,000 LOC**.
   - *Tindakan Preventif*: Pecah menjadi modul folder `src/core/systems/exchange/`:
     - `discovery.rs`: Spontaneous Knowledge & Eureka Breakthroughs.
     - `harvesting.rs`: Natural Resource Harvesting with Capital Tools.
     - `production.rs`: Roundabout Capital Good Manufacturing.
     - `services.rs`: Non-Rival Apprenticeship & Labor Time Delivery.
     - `barter.rs`: Bilateral Barter & Hayekian Market Intelligence.

2. **`src/core/domain/statistic/table_calculator.rs` (Saat ini: ~510 LOC)**:
   - *Prediksi Skala*: Setiap penambahan tabel indikator baru (PDB riil, indeks Gini, neraca perdagangan, matriks input-output) menambahkan ~150 LOC. Di fase peradaban lanjut (5-10 tabel), berkas akan membengkak hingga **> 1,500 LOC**.
   - *Tindakan Preventif*: Transformasi menjadi sub-modul `src/core/domain/statistic/calculators/`:
     - `demography_table.rs` (`TAB_DEMO_01`)
     - `commodity_table.rs` (`TAB_COMM_01`)
     - `item_catalogue_table.rs` (`TAB_ITEM_01`)
     - `mod.rs`: Re-export clean interface.

3. **`src/adapters/persistence/parquet_exporter.rs` (Saat ini: ~517 LOC)**:
   - *Prediksi Skala*: Setiap penambahan entitas domain baru membutuhkan Arrow Schema Builder dan RecordBatch serializer baru.
   - *Tindakan Preventif*: Pisahkan skema per tabel ke dalam sub-modul serializer (`schema/agent_schema.rs`, `schema/ledger_schema.rs`, `schema/table_schema.rs`).

4. **`src/main.rs` (Saat ini: ~370 LOC)**:
   - *Prediksi Skala*: Berkas composition root akan semakin padat seiring bertambahnya konfigurasi CLI, world generator, dan reporter.
   - *Tindakan Preventif*: Ekstrak seeding data awal ke `src/bootstrap/initial_world.rs` dan perenderan output ke `src/bootstrap/summary_view.rs`.

---

## 🛠️ CLI Toolkit: Script Audit Otomatis Skalabilitas

Repositori ini dilengkapi dengan tool audit otomatis untuk memindai seluruh berkas secara instan:

```bash
# Jalankan audit ukuran berkas dan pelanggaran SOLID
bash scripts/audit_solid_scale.sh
```

Atau menggunakan one-liner bash langsung:

```bash
# Deteksi berkas > 300 baris dalam src/
find src -name "*.rs" -exec wc -l {} + | sort -rn | awk '$1 > 300 { print "🔴 WARNING: " $2 " (" $1 " LOC) exceeds SOLID threshold" }'
```

---

## 📋 Checklist Eksekusi Saat Mereview atau Merancang Modul Baru

Sebelum commit atau menganggap fitur selesai, tanyakan 4 hal ini:
- [ ] Apakah berkas yang saya ubah/buat berada di bawah ambang batas **350 LOC**?
- [ ] Apakah berkas memiliki **hanya 1 tanggung jawab utama** (*Single Responsibility*)?
- [ ] Jika di masa depan ada 5 fitur baru sejenis ditambahkan, apakah berkas ini akan meledak ukurannya atau cukup menambah berkas baru secara terisolasi (*Open/Closed*)?
- [ ] Apakah pengujian unit (*unit test*) dapat ditulis secara bersih tanpa harus mempersiapkan *boilerplate* dari 10 modul lain?
