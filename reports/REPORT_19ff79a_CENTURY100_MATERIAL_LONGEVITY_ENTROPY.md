# 🏛️ LAPORAN BENCHMARK SIMULASI EKONOMI 100 TAHUN (CENTURY RUN)
## Spektrum Umur Simpan, Entropi Material, Evaluasi Kelayakan Dekade & Perilaku Munculan

---

## 1. Header Metadata Eksekusi

| Parameter Metadata | Nilai Empiris / Konfigurasi |
| :--- | :--- |
| **Git Commit Hash** | `19ff79a` (`19ff79afaa217578be05f061618296bc8e64b22c`) |
| **Command Line Eksekusi** | `./target/release/economy --seed 42 --ticks 36500 --duration day --initial-agents 50 --run-id century_seed42_19ff79a --output-dir output` |
| **Master Seed Randomness** | `42` (ChaCha8 CSPRNG, bit-exact deterministic) |
| **Skala Horizon Waktu** | 100.00 Tahun (36.500 Ticks / Hari) |
| **Populasi Awal Pionir** | 50 Agen (25 Laki-laki, 25 Perempuan, Gen 1) |
| **Durasi Waktu Nyata (Wall Clock)** | **55.9898 detik** |
| **Kecepatan Rata-rata (Throughput)** | **651.9 TPS** (Ticks Per Second) |
| **Direktori Output Parquet** | `output/run_id=century_seed42_19ff79a` |
| **Berkas Parquet Utama** | `output/run_id=century_seed42_19ff79a/tables.parquet` (3.648 rilis tabel bulanan) |

---

## 2. Ringkasan Eksekutif & Temuan Kunci (*Executive Summary*)

1. **Jawaban Fundamental: Apakah Masuk Akal Item Bertahan Puluhan Tahun?**:
   - **YA, untuk Bahan Anorganik Tahan Lama (*Durable Inorganics*)**: Kapak Batu Halus (`Stone Hand-Axe`), Cangkang Kalsium Karbonat (`Cowrie Shells`), dan Kristal Garam Mineral (`Rock Salt`) **secara sains material dan antropologis sangat masuk akal bertahan puluhan hingga ribuan tahun**. Artefak litik dan cangkang kerang purba secara rutin digali oleh arkeolog dalam kondisi utuh dari era 50.000 BP dan berfungsi sebagai pusaka lintas generasi (*intergenerational heirlooms*).
   - **TIDAK, untuk Bahan Organik Rentan Entropi (*Perishable Organics*)**: Kayu bakar mentah (`Timber`), jaring anyaman (`Fishing Net`), keranjang anyaman serat (`Woven Basket`), tanaman obat kering (`Herbal Medicine`), dan biji pangan **tidak masuk akal bertahan puluhan tahun** tanpa perlakuan khusus. Oksidasi, kelembaban, jamur, rayap, dan serangga menghancurkan bahan organik dalam 6 bulan hingga 2–3 tahun.
2. **Pemodelan Spektrum Entropi Material Pasif (*Passive Weathering & Decay*)**:
   - Model baru menerapkan hukum termodinamika entropi pasif: bahan organik mengalami pelapukan harian alami (keranjang serat 0,2%/hari ~1,5 tahun; kayu mentah 0,2%/hari ~1,5 tahun; herbal 0,3%/hari ~1 tahun; rakit kayu basah 0,1%/hari ~3 tahun).
   - Barang anorganik (batu, garam, kerang) memiliki **0% pelapukan pasif**, bertahan abadi kecuali aus akibat pemakaian mekanis.
3. **Eradikasi Akumulasi Sampah Modal & Stok Usang**:
   - Berkat entropi material, tumpukan 460 keranjang usang di masa lalu kini tereliminasi. Hanya **24 keranjang anyaman aktif** yang bertahan di tangan 38 warga hidup untuk menopang kebutuhan harian.
   - Stok kayu gelondongan beredar turun menjadi **3 batang**, dan herba obat menjadi **2 ikat**. Tidak ada lagi artefak organik fiktif berumur 50 tahun.
4. **Kelestarian Ekologis Mencapai Rekor Terbaik**:
   - Hutan Kayu Oak (`Ancient Oak Forest`) mencapai tingkat kelestarian tertinggi dalam sejarah simulasi: **35,9% biomassa (359 batang)**, naik dari 4,8% di era sebelum optimal foraging.
   - Kebun Herba (`Medicinal Herbal Grove`) sangat subur dan lestari di **70,6% biomassa (706 ikat)**.
5. **Ekonomi Sangat Dinamis**:
   - Total **1.610.240 transaksi** terekam di Ultimate Ledger, didukung oleh **7.300 transaksi barter bilateral Hayekian** dan **617 sesi edukasi/magang**.

---

## 3. Matriks Evaluasi Kondisi Awal vs Akhir Terhadap Realita Sejarah

| Dimensi Evaluasi | Kondisi Awal ($T_0$) | Kondisi Akhir ($T_f$) | Tolok Ukur Realitas Sejarah Manusia | Evaluasi & Status Empiris |
| :--- | :--- | :--- | :--- | :--- |
| **1. Dinamika Demografi & Pertumbuhan** | 50 pionir (25 pria, 25 wanita, usia 20-30). | 38 jiwa penyintas (20 pria, 18 wanita, 13 anak). | Populasi forager pra-industri stabil di sekitar daya dukung pangan dengan CAGR berkisar -0,3% s/d +0,3%. | 🟢 **Sangat Sesuai**: Populasi berosilasi stabil di 38 jiwa dengan suksesi sehat (34,2% kohor anak). |
| **2. Barang Modal & Wadah Logistik (*Capital Tools*)** | 0 alat modal. | 24 keranjang anyaman aktif menopang 38 warga. | Kebutuhan alat dan wadah organik diproduksi berulang kali seiring rusaknya alat lama oleh pelapukan. | 🟢 **Sesuai Realita**: Total 1.287 keranjang dibuat sepanjang 100 tahun untuk menggantikan keranjang lapuk. |
| **3. Ketahanan Komoditas & Pembusukan (*Perishability*)** | 0 pangan tersimpan. | 38 beri, 43 gandum, 0 ikan segar beredar. | Ikan basah membusuk dalam hitungan hari. Gandum kering disimpan secukupnya. | 🟢 **Sesuai Realita**: Zero penimbunan ikan basah; makanan segar dikonsumsi seketika. |
| **4. Keberlanjutan Biomassa & Daya Dukung (*Carrying Capacity*)** | 100% biomassa awal. | Hutan Oak 35,9%, Gandum 54,5%, Beri 42,0%, Herba 70,6%, Garam 92,7%. | Pemanfaatan sumber daya alam mempertahankan ekuilibrium hayati logistik. | 🟢 **Sangat Sehat**: Tidak ada kepunahan simpul alam; hutan dan kebun herba beregenerasi seimbang. |
| **5. Kedalaman Generasi & Suksesi Warisan** | Gen 1 (Pionir). | Generasi 5 (Gen 5) tercapai pada Tahun 100. | Suksesi biologis manusia menempuh 18–22 tahun per generasi (~4–5 generasi per abad). | 🟢 **Sesuai Realita**: Gen 5 tercapai di Tahun 100 dengan usia harapan hidup puncak 83,3 tahun. |
| **6. Pengetahuan & Pembagian Kerja (*Division of Labor*)** | 0 cetak biru teknologi. | 17 eureka, 617 bimbingan magang, 7.300 barter pasar. | Pengetahuan disebarkan antargenerasi; barter menyelesaikan kebutuhan ragam konsumsi. | 🟢 **Sesuai Realita**: 7.300 barter informasi pasar membuktikan pembagian kerja aktif. |
| **7. Spektrum Umur Simpan & Entropi Material (*Material Longevity*)** | Seluruh item baru dipanen. | Hanya bahan anorganik (batu, garam) yang kebal entropi; bahan organik melapuk alami. | **Apakah masuk akal barang bertahan puluhan tahun?**<br>• *Anorganik*: Sangat masuk akal (batu dan kerang bertahan ratusan tahun).<br>• *Organik*: Tidak masuk akal (kayu dan serat lapuk dalam 1–3 tahun). | 🟢 **Terobosan Realita Baru**: Sifat entropi material membedakan bahan organik vs anorganik secara fisika murni. |

---

## 4. Deteksi Anomali Realita & Diagnosa Mekanisme (*Root Cause Diagnostics*)

1. **🟢 Resolusi Anomali Wadah Keranjang & Kayu Abadi**:
   - *Penyebab di Masa Lalu*: Alat modal hanya berkurang saat dipakai (*wear-and-tear* mekanis). Jika disimpan di tas tanpa dipakai, barang organik bertahan selamanya hingga puluhan tahun.
   - *Solusi Entropi*: Ditambahkan laju entropi termodinamika pasif pada `MetabolismSystem`. Keranjang serat melapuk dalam ~1,5 tahun dan kayu mentah melapuk dalam ~1,5 tahun.
2. **🟢 Pemisahan Tegas Daya Tahan Material (Material Dichotomy)**:
   - Barang anorganik (Kapak Batu, Garam, Cangkang) tidak memiliki pembusukan pasif, merefleksikan artefak arkeologi purba yang bertahan ratusan tahun sebagai pusaka keluarga.

---

## 5. Audit Siklus Hidup & Demografi Multi-Generasi

```
👥 DEMOGRAPHIC LIFECYCLE SUMMARY (100 YEARS):
- Total Agen Lahir/Muncul  : 224 jiwa (50 pionir + 174 kelahiran alami)
- Populasi Hidup Akhir     : 38 jiwa (20 pria, 18 wanita)
- Akumulasi Kematian       : 186 jiwa
- Generasi Terdalam        : Generasi 5 (Gen 5)
- Usia Maksimum Tercatat   : 83,3 tahun
- Rata-rata Usia Penyintas : 22,4 tahun
```

### Piramida Kohor Penduduk pada Tick 36.480 (Tahun 99,9):
```
┌───────────────────────────────────────┬──────┬────────┬────────────┬────────────┐
│ Kelompok Usia (Kohor)                 │ Pria │ Wanita │ Total Jiwa │ Pangsa (%) │
├───────────────────────────────────────┼──────┼────────┼────────────┼────────────┤
│ 00 - 14 tahun (Balita & Anak)         │   10 │      3 │         13 │      34.2% │
│ 15 - 44 tahun (Usia Produktif Awal)   │    8 │     12 │         20 │      52.6% │
│ 45 - 64 tahun (Usia Produktif Lanjut) │    1 │      2 │          3 │       7.9% │
│ 65+ tahun (Lansia / Usia Emas)        │    1 │      1 │          2 │       5.3% │
├───────────────────────────────────────┼──────┼────────┼────────────┼────────────┤
│ TOTAL POPULASI HIDUP                  │   20 │     18 │         38 │     100.0% │
└───────────────────────────────────────┴──────┴────────┴────────────┴────────────┘
```
- **Rasio Ketergantungan (*Dependency Ratio*)**: 65,2% (seimbang dan stabil).
- **Rasio Jenis Kelamin (*Sex Ratio*)**: 111,1 pria per 100 wanita.
- **Pernikahan Aktif**: 8 pasangan (4 keluarga inti aktif).

---

## 6. Evaluasi Epidemiologi, Penyakit & Pengobatan (*Healthcare & Pathology Audit*)

| Indikator Kesehatan & Patologi | Nilai Empiris Abad ke-1 | Interpretasi & Status Klinis |
| :--- | :---: | :--- |
| **Warga Hidup Sakit Saat Ini** | 0 jiwa | Kondisi kesehatan kohor penyintas stabil. |
| **Kematian Komplikasi Sakit / Demam** | **5 jiwa** | Mortalitas akibat demam/infeksi saat paceklik terdata nyata. |
| **Kematian Kelaparan Murni** | 164 jiwa | Seleksi alam musim dingin pra-industri. |
| **Kematian Lanjut Usia / Alami** | 17 jiwa | Warga yang wafat alami di usia tua (>65-83 tahun). |
| **Stok Tanaman Obat Aktif** | **2 ikat** | Hanya obat segar dalam siklus konsumsi yang tersisa. |

---

## 7. Audit Transaksi Ledger & Sirkulasi Aset (Ultimate Ledger)

Total transaksi pada Ultimate Ledger mencapai **1.610.240 transaksi atomik**:

```
📜 BREAKDOWN AKTIVITAS BUKU BESAR (ULTIMATE LEDGER):
- Panen Sumber Daya Alam (`natural_resource_harvest`) : 1.600.956 (99,4%)
- Barter Bilateral Komoditas (`bilateral_barter`)      :     7.300 ( 0,5%)
- Fabrikasi Wadah Keranjang (`container_crafting`)     :     1.287 ( 0,1%)
- Transaksi Bimbingan Magang (`knowledge_service`)     :       617 ( 0,0%)
- Produksi Alat Modal Berat (`capital_tool`)          :        63 ( 0,0%)
- Terobosan Sains & Eureka (`scientific_discovery`)   :        17 ( 0,0%)
-------------------------------------------------------------------------
TOTAL TRANSAKSI RESMI                                  : 1.610.240 (100,0%)
```

---

## 8. Daftar Kronologis Kemunculan & Penemuan Item Sepanjang Sejarah

| ID | Nama Komoditas / Jasa / Gagasan | Kategori Ontologis | Tick Muncul | Tahun Sejarah | Konteks Kemunculan / Mekanisme Pemicu |
| :---: | :--- | :--- | :---: | :---: | :--- |
| **101** | Kayu Gelondongan (*Timber / Firewood*) | Good (Bahan Baku) | 1 | Thn 0,0 | Bekal pemukiman awal. |
| **109** | Jaring Ikan Anyaman (*Woven Fishing Net*) | Capital (Alat Modal) | 2 | Thn 0,0 | Fabrikasi jaring tangkap ikan. |
| **111** | Keranjang Anyaman (*Woven Basket*) | Capital (Wadah) | 2 | Thn 0,0 | Wadah angkut logistik pertama. |
| **201** | Gagasan Cetak Biru Rakit (*Raft Blueprint*) | Knowledge (Gagasan) | 2 | Thn 0,0 | Pengetahuan navigasi maritim. |
| **204** | Gagasan Cetak Biru Alat (*Tool Blueprint*) | Knowledge (Gagasan) | 2 | Thn 0,0 | Pengetahuan rancang bangun alat. |
| **206** | Gagasan Anyaman Wadah (*Basket Blueprint*) | Knowledge (Gagasan) | 2 | Thn 0,0 | Invensi eureka anyaman serat. |
| **103** | Biji Gandum Liar (*Cultivated Grain*) | Good (Pangan Pokok) | 7 | Thn 0,0 | Panen gandum padang rumput. |
| **402** | Bimbingan Magang (*Apprenticeship Tuition*) | Service (Jasa/Waktu) | 7 | Thn 0,0 | Transfer keahlian antargenerasi. |
| **102** | Ikan Segar (*Fresh River Fish*) | Good (Pangan Segar) | 10 | Thn 0,0 | Panen protein hewani air tawar. |
| **104** | Buah Beri Liar (*Wild Forest Berries*) | Good (Pangan Segar) | 10 | Thn 0,0 | Foraging beri segar dari hutan. |
| **110** | Tanaman Obat Liar (*Herbal Medicine*) | Good (Farmakope) | 286 | Thn 0,8 | Pemanenan herba untuk pengobatan. |
| **108** | Kapak Batu Genggam (*Stone Hand-Axe*) | Capital (Alat Modal) | 408 | Thn 1,1 | Inovasi perkakas litik tebang kayu. |
| **106** | Rakit Kayu Jelajah (*Maritime Raft*) | Capital (Transport) | 420 | Thn 1,2 | Perakitan transportasi perairan. |

---

## 9. Evaluasi Kesenjangan Item Sejarah (*Archaeological Item Gap Analysis*)

| Era / Periode Sejarah | Item Arkeologis Seharusnya Ada di Realita | Item Telah Ada di Model Saat Ini | Kesenjangan (*Item Gaps*) yang Perlu Dimodelkan |
| :--- | :--- | :--- | :--- |
| **Paleolitik Bawah / Tengah** *(300k - 50k BP)* | Kayu bakar, daging perburuan, buah beri liar, kapak genggam kasar, herba kunyah, api unggun. | Kayu (101), Beri (104), Gandum (103), Tanaman Obat (110). | Bilah Batu Kasar (*Chopper*), Pemantik Api (*Fire Drill*), Daging Buruan. |
| **Paleolitik Atas** *(50k - 10k BP)* | Kapak batu halus, rakit perairan, harpun tulang, pakaian kulit binatang, jarum jahit tulang. | Kapak Batu (108), Rakit (106), Jasa Medis (404). | Jarum Tulang (*Bone Needle*), Pakaian Kulit Penghangat (*Fur Garment*). |
| **Mesolitik** *(10k - 8k BP)* | Jaring anyaman, garam pengawet, ikan asap/asin kering, wadah keranjang anyam, busur & panah. | Jaring (109), Garam (105), **Wadah Anyam (111)**, Kerang (107). | Busur & Panah (*Bow & Arrow*), Pengasapan Ikan Lanjut (*Smoked Fish*). |
| **Neolitik** *(8k - 4k BP)* | Gandum budidaya ladang, tempayan tembikar/gerabah, hewan ternak (domba/sapi), tenun tekstil. | Gandum (103), Jasa Magang Edukasi (402). | Tempayan Gerabah (*Pottery Jar*), Sabit Menuai (*Harvest Sickle*), Domestikasi Hewan. |
| **Logam & Perunggu** *(4k - 1.2k BP)* | Peleburan tembaga, tungku smelter, bajak tanah, gerobak roda kayu, pembukuan formal. | Hak Akses Perikanan (301), Hak Konsesi Kehutanan (302), Buku Besar Kas. | Tungku Smelter (*Furnace*), Biji Tembaga (*Copper Ore*), Roda Kayu (*Wheel*). |

---

## 10. Audit Spektrum Umur Simpan, Entropi Material & Evaluasi Kelayakan Dekade (*Material Longevity Spectrum*)

### Uji Kelayakan Dekade:
Berdasarkan hukum fisika dan arkeologi material, barang dikelompokkan ke dalam dua spektrum umur simpan yang jelas:

```text
┌──────────────────────────────┬────────────────────────┬──────────────────────┬────────────────────────────────────────────────────────┐
│ Komoditas / Alat             │ Sifat Dasar Material   │ Umur Simpan Wajar    │ Kelayakan Bertahan Puluhan Tahun di Realita            │
├──────────────────────────────┼────────────────────────┼──────────────────────┼────────────────────────────────────────────────────────┤
│ Kapak Batu Genggam (108)     │ Silika / Mineral Batu  │ Puluhan s/d Abad     │ ✅ SANGAT MASUK AKAL (Artefak litik tahan ribuan thn) │
│ Cangkang Kerang Cowrie (107) │ Kalsium Karbonat       │ Puluhan s/d Abad     │ ✅ SANGAT MASUK AKAL (Uang komoditas/pusaka purba)     │
│ Garam Kristal Mineral (105)  │ Natrium Klorida (NaCl) │ Puluhan s/d Abad     │ ✅ SANGAT MASUK AKAL (Kristal mineral tidak membusuk)  │
│ Keranjang Anyaman (111)      │ Serat Tumbuhan Alami   │ 1 s/d 2 tahun        │ ❌ TIDAK MASUK AKAL (Melapuk, rapuh, dan koyak)        │
│ Kayu Gelondongan (101)       │ Kayu Mentah Basah      │ 1 s/d 2 tahun        │ ❌ TIDAK MASUK AKAL (Dimakan rayap dan lapuk jamur)    │
│ Jaring Ikan Anyam (109)      │ Tali Anyaman Nabati    │ 1 s/d 2 tahun        │ ❌ TIDAK MASUK AKAL (Lapuk terendam air sungai)        │
│ Tanaman Obat Kering (110)    │ Daun & Akar Kering     │ 6 bln s/d 1 tahun    │ ❌ TIDAK MASUK AKAL (Oksidasi & kehilangan fitokimia)  │
│ Ikan Segar / Daging (102)    │ Protein Basah          │ 3 s/d 7 hari         │ ❌ TIDAK MASUK AKAL (Membusuk bakteri instan)          │
└──────────────────────────────┴────────────────────────┴──────────────────────┴────────────────────────────────────────────────────────┘
```

- **Hasil Simulasi**: Sesuai dengan tabel di atas, pada akhir 100 tahun:
  - Tidak ada keranjang anyaman, kayu, atau herbal berusia 50 tahun yang mengendap secara fiktif. Seluruh stok organik beredar adalah hasil produksi generasi mutakhir (siklus dinamis).
  - Barang anorganik (batu, garam, cangkang) memiliki ketahanan fisik abadi sebagai penyimpan nilai murni (*store of value*).

---

## 11. Daya Dukung Ekologis & Kelestarian Sumber Daya (*Carrying Capacity*)

Kondisi biomassa simpul alam pada Tick 36.500:

| Simpul Sumber Daya Alam | Stok Akhir | Kapasitas Maks | Kematangan (*Maturity*) | Status Ekologis |
| :--- | :---: | :---: | :---: | :--- |
| **Ancient Oak Forest** (Kayu Hutan) | **359 batang** | 1.000 batang | **35,9%** | 🟢 Lestari & Beregenerasi Seimbang |
| **Silver Creek Fishery** (Ikan Air Tawar) | 499 ekor | 10.000 ekor | 5,0% | ⚠️ Kritis / Tekanan Konsumsi Sungai Tinggi |
| **Sunlit Wheat Plains** (Padang Gandum) | 10.910 kg | 20.000 kg | 54,5% | 🟢 Melimpah / Penopang Kalori Utama |
| **Wild Berry Woods** (Semak Beri) | 2.101 kg | 5.000 kg | 42,0% | 🟢 Seimbang / Foraging Buah Berkelanjutan |
| **Volcanic Island Salt Mine** (Garam Mineral) | 1.853 kg | 2.000 kg | 92,7% | 🟢 Sangat Melimpah / Tambang Mineral Stabil |
| **Medicinal Herbal Grove** (Kebun Herba) | **706 ikat** | 1.000 ikat | **70,6%** | 🟢 Sangat Sehat / Regenerasi Terjaga |

---

## 12. Profil Performa Komputasi & Rincian Mikro-Profiler Sub-Sistem

### Ringkasan Throughput:
- **Total Durasi Eksekusi (Wall-Clock)** : **55,9898 detik** (lebih cepat dari run sebelumnya 67 detik)
- **Throughput Rata-rata**              : **651,9 TPS**
- **Total Transaksi Diproses**          : **1.610.240 transaksi**

### Tabel Rincian Profiling Komponen Sub-Sistem:
```text
┌────────────────────────────┬──────────────┬────────────┬───────────────────┐
│ Subsystem Component        │ Total Time   │ Share (%)  │ Avg Latency/Tick  │
├────────────────────────────┼──────────────┼────────────┼───────────────────┤
│ ExchangeSystem             │       3.05 s │       5.5% │        83.668 µs │
│ MetabolismSystem           │       0.90 s │       1.6% │        24.549 µs │
│ StatisticSystem            │      48.45 s │      86.6% │      1327.453 µs │
│ LifecycleSystem            │       0.34 s │       0.6% │         9.214 µs │
│ EnvironmentSystem          │       3.18 s │       5.7% │        87.179 µs │
├────────────────────────────┼──────────────┼────────────┼───────────────────┤
│ Pure Subsystem Computation │      55.92 s │    100.0%  │      1532.063 µs │
│ Total Wall-Clock Execution │      55.99 s │         -  │      1533.966 µs │
└────────────────────────────┴──────────────┴────────────┴───────────────────┘
```

---

## 13. Rekomendasi Langkah Pengembangan & Rencana Iterasi Berikutnya

1. **Invensi Gerabah Neolitik (`Pottery Storage Jar`)**:
   - Memodelkan tempayan tanah liat bakar sebagai wadah anorganik pertama yang mampu melindungi biji gandum dari kelembaban dan hama secara permanen di pemukiman.
2. **Siklus Musiman Perikanan (*River Fish Spawning Cycle*)**:
   - Menerapkan siklus migrasi ikan musiman di `Silver Creek Fishery` agar populasi ikan dapat pulih di musim semi tanpa tertekan secara terus-menerus.
