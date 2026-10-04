# 🏛️ LAPORAN BENCHMARK SIMULASI EKONOMI 100 TAHUN (CENTURY RUN)
## Optimal Foraging Berpusat pada Agen, Batasan Muatan Fisik, Wadah Keranjang Anyaman & Pertukaran Bilateral

---

## 1. Header Metadata Eksekusi

| Parameter Metadata | Nilai Empiris / Konfigurasi |
| :--- | :--- |
| **Git Commit Hash** | `d2590c9` (`d2590c946fdf39665bc7f4db2399a0db89d53c3e`) |
| **Command Line Eksekusi** | `./target/release/economy --seed 42 --ticks 36500 --duration day --initial-agents 50 --run-id century_seed42_d2590c9 --output-dir output` |
| **Master Seed Randomness** | `42` (ChaCha8 CSPRNG, bit-exact deterministic) |
| **Skala Horizon Waktu** | 100.00 Tahun (36.500 Ticks / Hari) |
| **Populasi Awal Pionir** | 50 Agen (25 Laki-laki, 25 Perempuan, Gen 1) |
| **Durasi Waktu Nyata (Wall Clock)** | **67.0641 detik** |
| **Kecepatan Rata-rata (Throughput)** | **544.3 TPS** (Ticks Per Second) |
| **Direktori Output Parquet** | `output/run_id=century_seed42_d2590c9` |
| **Berkas Parquet Utama** | `output/run_id=century_seed42_d2590c9/tables.parquet` (3.648 rilis tabel bulanan) |

---

## 2. Ringkasan Eksekutif & Temuan Kunci (*Executive Summary*)

1. **Munculnya *Emergent Behavior* Murni Tanpa Hardcode**:
   - Dengan mengganti iterasi lingkungan (*environment-driven loop*) menjadi **tindakan optimal berpusat pada agen (*agent-centric optimal foraging*)**, agen bertindak berdasarkan urgensi fisiologis (kelaparan, kedinginan, kesakitan) dan evaluasi hasil marjinal (*Charnov's Marginal Value Theorem*).
   - Hasilnya, agen tidak lagi mengeksploitasi sumber daya secara membabi buta. Penebangan hutan Oak berhenti saat biomassa menipis, memulihkan kelestarian hutan dari 4,8% melonjak ke **32,4%**.
2. **Invensi & Adopsi Massal Wadah Anyaman (*Woven Basket Containers*)**:
   - Pembatasan muatan fisik realistis ($W_{base} = 25$ kg) memicu kebutuhan logistik. Gagasan anyaman wadah (`KNOWLEDGE_BASKET_WEAVING`, ID 206) ditemukan secara spontan pada **Tick 3 (Tahun 0,0)** dan keranjang anyaman pertama (`WOVEN_BASKET`, ID 111) dibuat pada **Tick 7**.
   - Sepanjang satu abad, masyarakat memproduksi **1.163 keranjang anyaman** secara mandiri untuk memperluas daya angkut logistik (+25 kg per wadah).
3. **Eradikasi Total Anomali Penimbunan Herbal**:
   - Anomali penimbunan 20.468 tanaman obat pada iterasi sebelumnya berhasil **teratasi 100%**. Stok herbal beredar turun ke angka realistis **170 ikat** (rata-rata 3,2 per kapita, maksimal 8 ikat per agen), dan simpul `Medicinal Herbal Grove` tetap lestari di 74,2%.
4. **Ledakan Transaksi Barter Bilateral & Edukasi Magang**:
   - Pembatasan muatan fisik mendorong agen melepaskan surplus barang melalui perdagangan. Transaksi barter bilateral melonjak **lebih dari 100 kali lipat** dari 68 transaksi menjadi **7.423 transaksi barter Hayekian**.
   - Transfer pengetahuan antargenerasi (magang) meningkat 2,5 kali lipat menjadi **698 sesi bimbingan**.
   - Total transaksi Ultimate Ledger mencapai **1.794.844 transaksi**.
5. **Vitalitas Demografi Meningkat**:
   - Populasi penyintas di Tahun 100 melonjak ke **53 jiwa** (dibandingkan 37 jiwa pada commit sebelumnya), dengan **193 kelahiran alami** (total 243 agen historis), mencapai **Generasi 6 (Gen 6)**.

---

## 3. Matriks Evaluasi Kondisi Awal vs Akhir Terhadap Realita Sejarah

| Dimensi Evaluasi | Kondisi Awal ($T_0$) | Kondisi Akhir ($T_f$) | Tolok Ukur Realita Sejarah Manusia | Evaluasi & Status Empiris |
| :--- | :--- | :--- | :--- | :--- |
| **1. Dinamika Demografi & Pertumbuhan** | 50 pionir (25 pria, 25 wanita, usia 20-30). | 53 jiwa penyintas (22 pria, 31 wanita, 26 anak). | Masyarakat pemburu-peramu/agraris memiliki pertumbuhan tahunan berkisar 0,0% s/d 0,3% dengan pergantian generasi alami. | 🟢 **Sangat Sesuai Realita**: CAGR tercatat +0,06%/tahun, populasi tumbuh alami dari 50 ke 53 jiwa dengan piramida usia sehat (49,1% kohor muda). |
| **2. Barang Modal & Wadah Logistik (*Containers & Tools*)** | 0 alat modal, 0 wadah penyimpanan. | 460 keranjang anyaman, 18 jaring ikan bertahan dari 1.278 alat diproduksi. | Transisi Mesolitik ke Neolitik ditandai invensi wadah anyaman keranjang dan gerabah untuk mengangkut dan menyimpan pangan. | 🟢 **Sesuai Realita**: Munculnya 1.163 keranjang anyaman membuktikan mekanisme adaptasi terhadap batasan muatan biologis tubuh. |
| **3. Ketahanan Komoditas & Pembusukan (*Perishability*)** | 0 pangan segar tersimpan. | 35 beri liar, 53 gandum, 0 ikan segar beredar. | Ikan segar cepat membusuk tanpa garam. Buah beri hanya bertahan beberapa hari. | 🟢 **Sesuai Realita**: Tidak ada penimbunan ikan basah abadi. Beri dan gandum dikonsumsi teratur. |
| **4. Keberlanjutan Biomassa & Daya Dukung (*Carrying Capacity*)** | 100% biomassa awal. | Hutan Oak 32,4%, Perikanan 5,0%, Ladang Gandum 40,1%, Beri 38,8%, Herba 74,2%, Garam 92,3%. | Pemanfaatan sumber daya alam harus menghasilkan rotasi foraging dan pencegahan eksploitasi mutlak saat stok menipis. | 🟢 **Peningkatan Signifikan**: Teorema Nilai Marjinal Charnov menyelamatkan Hutan Oak dari 4,8% menjadi 32,4%. |
| **5. Kedalaman Generasi & Suksesi Warisan** | Gen 1 (Pioneer Settlers). | Generasi 6 (Gen 6) tercapai pada Tahun 100. | Pergantian generasi biologis manusia menempuh 16–22 tahun per generasi. | 🟢 **Sesuai Realita**: Gen 6 dalam 100 tahun membuktikan kontinuitas garis keturunan multi-generasi tanpa distorsi. |
| **6. Pengetahuan & Pembagian Kerja (*Division of Labor*)** | 0 cetak biru teknologi (autarki dasar). | 23 terobosan eureka, 698 sesi magang, 7.423 barter pasar. | Spesialisasi muncul ketika agen bertukar surplus untuk memenuhi kebutuhan ragam pangan dan alat. | 🟢 **Sesuai Realita**: Barter bilateral melonjak 109x lipat membuktikan ketergantungan pasar organik. |

---

## 4. Deteksi Anomali Realita & Diagnosa Mekanisme (*Root Cause Diagnostics*)

1. **🟢 Resolusi Tuntas Anomali Penimbunan 20.468 Herbal**:
   - *Penyebab di Masa Lalu*: Tidak adanya batas berat inventori dan loop pemanenan berbasis lingkungan memaksa agen memungut herbal setiap hari tanpa batas.
   - *Solusi*: Batas beban angkut ($W_{base} = 25$ kg) dan *Optimal Foraging Theory* membuat agen hanya mengambil herbal saat sakit atau butuh cadangan kecil (1–3 ikat). Stok beredar turun menjadi **170 ikat**.
2. **🟢 Resolusi Anomali Eksploitasi Mutlak Hutan (*Deforestation Trap*)**:
   - *Penyebab di Masa Lalu*: Agen memanen pohon Oak terus-menerus terlepas dari stok yang menipis.
   - *Solusi*: Rasio kelimpahan stok $\frac{S}{K}$ menurunkan bobot pemilihan simpul yang hampir habis. Kematangan hutan Oak meningkat dari **4,8% menjadi 32,4%**.
3. **🟡 Anomali Tekanan Simpul Perikanan Sungai (`Silver Creek Fishery`)**:
   - *Observasi*: Stok perikanan sungai bertahan di 5,0% (499/10000 ekor).
   - *Diagnosa*: Ikan memiliki kalori tinggi (500 kkal) dan jaring ikan melipatgandakan panen 3x, sehingga bagi agen lapar di sekitar sungai, nilai marjinal ikan tetap tinggi.
   - *Rekomendasi*: Tambahkan biaya energi mobilitas menyeberang sungai atau musim paceklik bertelur (*spawning season*) di mana penangkapan ikan menghasilkan hasil lebih rendah.

---

## 5. Audit Siklus Hidup & Demografi Multi-Generasi

```
👥 DEMOGRAPHIC LIFECYCLE SUMMARY (100 YEARS):
- Total Agen Lahir/Muncul  : 243 jiwa (50 pionir + 193 kelahiran alami)
- Populasi Hidup Akhir     : 53 jiwa (22 pria, 31 wanita)
- Akumulasi Kematian       : 190 jiwa
- Generasi Terdalam        : Generasi 6 (Gen 6)
- Usia Maksimum Tercatat   : 74,6 tahun
- Rata-rata Usia Penyintas : 23,9 – 24,0 tahun
```

### Piramida Kohor Penduduk pada Tick 36.480 (Tahun 99,9):
```
┌───────────────────────────────────────┬──────┬────────┬────────────┬────────────┐
│ Kelompok Usia (Kohor)                 │ Pria │ Wanita │ Total Jiwa │ Pangsa (%) │
├───────────────────────────────────────┼──────┼────────┼────────────┼────────────┤
│ 00 - 14 tahun (Balita & Anak)         │   15 │     11 │         26 │      49.1% │
│ 15 - 44 tahun (Usia Produktif Awal)   │    3 │     13 │         16 │      30.2% │
│ 45 - 64 tahun (Usia Produktif Lanjut) │    2 │      5 │          7 │      13.2% │
│ 65+ tahun (Lansia / Usia Emas)        │    2 │      2 │          4 │       7.5% │
├───────────────────────────────────────┼──────┼────────┼────────────┼────────────┤
│ TOTAL POPULASI HIDUP                  │   22 │     31 │         53 │     100.0% │
└───────────────────────────────────────┴──────┴────────┴────────────┴────────────┘
```
- **Rasio Ketergantungan (*Dependency Ratio*)**: 130,4% (tingginya kelahiran balita 49,1% mencerminkan regenerasi pesat).
- **Rasio Jenis Kelamin (*Sex Ratio*)**: 71,0 pria per 100 wanita.
- **Pernikahan Aktif**: 18 pasangan (9 keluarga inti aktif).

---

## 6. Evaluasi Epidemiologi, Penyakit & Pengobatan (*Healthcare & Pathology Audit*)

| Indikator Kesehatan & Patologi | Nilai Empiris Abad ke-1 | Interpretasi & Status Klinis |
| :--- | :---: | :--- |
| **Warga Hidup Sakit Saat Ini** | 0 jiwa | Kondisi kesehatan kohor penyintas terkontrol baik. |
| **Kematian Komplikasi Sakit / Demam** | **2 jiwa** | Terdeteksi mortalitas akibat infeksi demam saat malnutrisi. |
| **Kematian Kelaparan Murni** | 176 jiwa | Paceklik musiman tetap menjadi faktor seleksi alam dominan. |
| **Kematian Lanjut Usia / Alami** | 12 jiwa | Lansia meninggal secara alami di usia >65 tahun. |
| **Stok Tanaman Obat Tersimpan** | **170 ikat** | Normal dan proporsional (turun drastis dari 20.468 ikat). |
| **Pembuatan Keranjang Logistik Obat** | 1.163 unit | Mendukung kapasitas distribusi logistik warga. |

---

## 7. Audit Transaksi Ledger & Sirkulasi Aset (Ultimate Ledger)

Total transaksi pada Ultimate Ledger melonjak drastis menjadi **1.794.844 transaksi atomik**:

```
📜 BREAKDOWN AKTIVITAS BUKU BESAR (ULTIMATE LEDGER):
- Panen Sumber Daya Alam (`natural_resource_harvest`) : 1.785.422 (99,5%)
- Barter Bilateral Komoditas (`bilateral_barter`)      :     7.423 ( 0,4%)
- Fabrikasi Wadah Keranjang (`container_crafting`)     :     1.163 ( 0,1%)
- Transaksi Bimbingan Magang (`knowledge_service`)     :       698 ( 0,0%)
- Produksi Alat Modal Berat (`capital_tool`)          :       115 ( 0,0%)
- Terobosan Sains & Eureka (`scientific_discovery`)   :        23 ( 0,0%)
-------------------------------------------------------------------------
TOTAL TRANSAKSI RESMI                                  : 1.794.844 (100,0%)
```

- **Alat Modal & Wadah yang Diproduksi**:
  - Keranjang Anyaman Wadah Angkut (`Woven Basket`): 1.163 unit dibuat, 460 unit bertahan.
  - Jaring Ikan Anyaman (`Woven Fishing Net`): 86 unit dibuat, 18 unit bertahan.
  - Kapak Batu Genggam (`Stone Hand-Axe`): 17 unit dibuat.
  - Rakit Jelajah Maritim (`Maritime Raft`): 12 unit dibuat.
- **Dinamika Pasar Hayekian**:
  - Seluruh 7.423 transaksi barter dilakukan secara *Informed Market Arbitrage* (0 transaksi buta), di mana agen menukar komoditas berlebih untuk mendapatkan komoditas yang utilitas marjinalnya lebih tinggi.

---

## 8. Daftar Kronologis Kemunculan & Penemuan Item Sepanjang Sejarah

| ID | Nama Komoditas / Jasa / Gagasan | Kategori Ontologis | Tick Muncul | Tahun Sejarah | Konteks Kemunculan / Mekanisme Pemicu |
| :---: | :--- | :--- | :---: | :---: | :--- |
| **101** | Kayu Gelondongan (*Timber / Firewood*) | Good (Bahan Baku) | 1 | Thn 0,0 | Bekal awal pemukiman pionir. |
| **201** | Gagasan Cetak Biru Rakit (*Raft Blueprint*) | Knowledge (Gagasan) | 1 | Thn 0,0 | Pengetahuan awal pionir navigasi. |
| **109** | Jaring Ikan Anyaman (*Woven Fishing Net*) | Capital (Alat Modal) | 2 | Thn 0,0 | Fabrikasi alat tangkap ikan pertama. |
| **204** | Gagasan Rancang Bangun Alat (*Tool Blueprint*) | Knowledge (Gagasan) | 2 | Thn 0,0 | Pengetahuan konstruksi perkakas. |
| **103** | Biji Gandum Liar (*Cultivated Grain*) | Good (Pangan Pokok) | 3 | Thn 0,0 | Panen gandum ladang pertama. |
| **206** | **Gagasan Anyaman Wadah (*Basket Blueprint*)** | **Knowledge (Gagasan)** | **3** | **Thn 0,0** | **Invensi eureka wadah logistik.** |
| **402** | Bimbingan Magang (*Apprenticeship Tuition*) | Service (Jasa/Waktu) | 3 | Thn 0,0 | Edukasi transfer pengetahuan perdana. |
| **111** | **Keranjang Anyaman (*Woven Carrying Basket*)** | **Capital (Wadah)** | **7** | **Thn 0,0** | **Fabrikasi wadah logistik perdana.** |
| **102** | Ikan Segar (*Fresh River Fish*) | Good (Pangan Segar) | 10 | Thn 0,0 | Hasil tangkapan protein sungai. |
| **104** | Buah Beri Liar (*Wild Forest Berries*) | Good (Pangan Segar) | 10 | Thn 0,0 | Foraging buah beri dari semak belukar. |
| **110** | Tanaman Obat Liar (*Herbal Medicine*) | Good (Farmakope) | 276 | Thn 0,8 | Pemanenan herba obat untuk perawatan. |
| **108** | Kapak Batu Genggam (*Stone Hand-Axe*) | Capital (Alat Modal) | 19.190 | Thn 52,6 | Fabrikasi kapak batu era pertengahan. |
| **106** | Rakit Kayu Jelajah (*Maritime Raft*) | Capital (Transport) | 19.194 | Thn 52,6 | Perakitan kendaraan perairan. |

---

## 9. Evaluasi Kesenjangan Item Sejarah (*Archaeological Item Gap Analysis*)

| Era / Periode Sejarah | Item Arkeologis Seharusnya Ada di Realita | Item Telah Ada di Model Saat Ini | Kesenjangan (*Item Gaps*) yang Perlu Dimodelkan |
| :--- | :--- | :--- | :--- |
| **Paleolitik Bawah / Tengah** *(300k - 50k BP)* | Kayu bakar, daging perburuan, buah beri liar, kapak genggam kasar, herba kunyah, api unggun. | Kayu (101), Beri (104), Gandum (103), Tanaman Obat (110). | Bilah Batu Kasar (*Chopper*), Pemantik Api (*Fire Drill*), Daging Buruan. |
| **Paleolitik Atas** *(50k - 10k BP)* | Kapak batu halus, rakit kayu, harpun tulang, pakaian kulit binatang, jarum jahit tulang. | Kapak Batu (108), Rakit (106), Jasa Medis (404). | Jarum Tulang (*Bone Needle*), Pakaian Kulit Penghangat (*Fur Garment*). |
| **Mesolitik** *(10k - 8k BP)* | Jaring anyaman, garam pengawet, ikan asap/asin kering, wadah keranjang anyam, busur & panah. | Jaring (109), Garam (105), **Wadah Anyam (111)**, Kerang (107). | Busur & Panah (*Bow & Arrow*), Pengasapan Ikan Lanjut (*Smoked Fish*). |
| **Neolitik** *(8k - 4k BP)* | Gandum budidaya ladang, tempayan tembikar/gerabah, hewan ternak (domba/sapi), tenun tekstil. | Gandum (103), Jasa Magang Edukasi (402). | Tempayan Gerabah (*Pottery Jar*), Sabit Menuai (*Harvest Sickle*), Domestikasi Hewan. |
| **Logam & Perunggu** *(4k - 1.2k BP)* | Peleburan tembaga, tungku smelter, bajak tanah, gerobak roda kayu, pembukuan formal. | Hak Akses Perikanan (301), Hak Konsesi Kehutanan (302), Buku Besar Kas. | Tungku Smelter (*Furnace*), Biji Tembaga (*Copper Ore*), Roda Kayu (*Wheel*). |

*Status Kesenjangan: `Keranjang Anyaman (111)` berhasil diisi dan dihapus dari daftar kesenjangan.*

---

## 10. Daya Dukung Ekologis & Kelestarian Sumber Daya (*Carrying Capacity*)

Kondisi biomassa simpul alam pada Tick 36.500:

| Simpul Sumber Daya Alam | Stok Akhir | Kapasitas Maks | Kematangan (*Maturity*) | Perubahan Status dari Commit Sebelumnya |
| :--- | :---: | :---: | :---: | :--- |
| **Ancient Oak Forest** (Kayu Hutan) | **324 batang** | 1.000 batang | **32,4%** | 🟢 **Pulih Drastis** (Sebelumnya 4,8% / Kritis) |
| **Silver Creek Fishery** (Ikan Air Tawar) | 499 ekor | 10.000 ekor | 5,0% | ⚠️ Kritis / Tekanan Konsumsi Protein Tinggi |
| **Sunlit Wheat Plains** (Padang Gandum) | 8.021 kg | 20.000 kg | 40,1% | 🟢 Sehat / Sumber Pangan Primer Terjaga |
| **Wild Berry Woods** (Semak Beri) | 1.941 kg | 5.000 kg | 38,8% | 🟢 Seimbang / Foraging Musiman Normal |
| **Volcanic Island Salt Mine** (Garam Mineral) | 1.846 kg | 2.000 kg | 92,3% | 🟢 Sangat Melimpah / Cadangan Stabil |
| **Medicinal Herbal Grove** (Kebun Herba) | **742 ikat** | 1.000 ikat | **74,2%** | 🟢 **Sangat Sehat** (Sebelumnya Dieksploitasi 20k) |

---

## 11. Profil Performa Komputasi & Rincian Mikro-Profiler Sub-Sistem

### Ringkasan Throughput:
- **Total Durasi Eksekusi (Wall-Clock)** : **67,0641 detik**
- **Throughput Rata-rata**              : **544,3 TPS**
- **Total Transaksi Diproses**          : **1.794.844 transaksi** (naik 15x dari 113k)

### Tabel Rincian Profiling Komponen Sub-Sistem:
```
┌────────────────────────────┬──────────────┬────────────┬───────────────────┐
│ Subsystem Component        │ Total Time   │ Share (%)  │ Avg Latency/Tick  │
├────────────────────────────┼──────────────┼────────────┼───────────────────┤
│ ExchangeSystem             │       3.70 s │       5.5% │       101.275 µs │
│ MetabolismSystem           │       1.10 s │       1.6% │        30.039 µs │
│ StatisticSystem            │      58.40 s │      87.2% │      1599.950 µs │
│ LifecycleSystem            │       0.42 s │       0.6% │        11.609 µs │
│ EnvironmentSystem          │       3.37 s │       5.0% │        92.424 µs │
├────────────────────────────┼──────────────┼────────────┼───────────────────┤
│ Pure Subsystem Computation │      66.99 s │    100.0%  │      1835.296 µs │
│ Total Wall-Clock Execution │      67.06 s │         -  │      1837.374 µs │
└────────────────────────────┴──────────────┴────────────┴───────────────────┘
```

### Diagnosis Bottleneck Komputasi:
1. **Peningkatan Skala Ledger Parquet (`StatisticSystem` 87,2%)**:
   - `StatisticSystem` memakan 58,40 detik karena volume transaksi Ultimate Ledger melonjak 15 kali lipat (dari 113.837 menjadi 1.794.844 transaksi). Setiap transaksi diindeks dan dihitung pada buletin statistik bulanan.
2. **Kinerja Logika Simulasi Inti Tetap Sangat Cepat**:
   - Seluruh logika agen (`ExchangeSystem` + `MetabolismSystem` + `LifecycleSystem` + `EnvironmentSystem`) hanya menghabiskan **8,59 detik** untuk 1,79 juta transaksi sepanjang 36.500 ticks (~235 µs per tick).
   - Arsitektur zero-copy dan modularitas domain terbukti sangat stabil menangani ledakan aktivitas ekonomi.

---

## 12. Rekomendasi Langkah Pengembangan & Rencana Iterasi Berikutnya

1. **Pemodelan Keranjang/Wadah Statis (Tempayan Gerabah / Granary Storage)**:
   - Keranjang anyaman (`Woven Basket`) memecahkan masalah muatan bergerak (+25 kg). Langkah berikutnya adalah mengadopsi gerabah neolitik (`Pottery Storage Jar`) sebagai wadah statis di lokasi pemukiman untuk menyimpan cadangan gandum kering hingga 100 kg.
2. **Optimalisasi Pengindeksan Transaksi Buku Besar**:
   - Mengingat volume transaksi melonjak mendekati 2 juta entri, agregasi metrik statistik dapat menggunakan ringkasan inkremental (*running accumulator*) daripada pemindaian ulang tabel transaksi bulanan.
3. **Regulasi Musiman Perikanan (*Seasonal Spawning Cycle*)**:
   - Menambahkan variasi musim bertelur pada simpul perikanan sungai agar biomassa ikan dapat pulih di musim semi tanpa tertekan secara persisten di 5%.
