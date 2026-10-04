---
name: abm-reality-auditor
description: Audit and verify Agent-Based Models (ABM) against biophysical, geographical, ecological, and climatic reality. Prevents naive uniform assumptions (such as uniform 4-season climate everywhere) by enforcing Köppen-Geiger spatial heterogeneity, environmental lapse rates, metabolic thermodynamic constraints, Liebig's law, and Pattern-Oriented Modeling (POM) validation.
---

# 🌍 ABM Reality-Auditor Skill: Biophysical & Empirical Realism SOP

Skill ini digunakan untuk mengaudit dan memastikan bahwa simulasi **Agent-Based Modeling (ABM)** di repositori `economy` berpijak pada **realitas biofisik, geografi spasial, iklim heterogen, dan termodinamika nyata**, serta mengeliminasi simplifikasi malas (seperti mengasumsikan seluruh dunia memiliki iklim 4 musim yang seragam).

---

## 🧭 Mengapa Audit Realitas Ini Krusial?

Dalam sains simulasi, kegagalan realitas terjadi ketika modeler memberlakukan aturan global seragam yang bertentangan dengan hukum geofisika bumi:
- **Contoh Kesalahan Fatal**: Mengatur seluruh grid dunia mengalami Musim Gugur & Musim Dingin bersalju secara serentak, padahal peradaban manusia awal berkembang di lembah sungai tropis/subtropis yang tidak pernah mengalami salju melainkan siklus Monsun Hujan & Kemarau.
- **Konsekuensi Buruk**: Agen mengembangkan adaptasi palsu (misal: menimbun kayu bakar untuk musim dingin di khatulistiwa), merusak validitas empiris simulasi ekonomi.

---

## 📋 6 Dimensi Audit Realitas Biofisik (The 6 Reality Dimensions)

### Dimensi 1: Heterogenitas Iklim Spasial & Klasifikasi Köppen-Geiger
- [ ] **Audit Aturan Iklim Seragam**: Apakah iklim dimodelkan sebagai satu variabel global tunggal untuk seluruh dunia?
  - *Red Flag*: `let climate = ClimateState::default_spring();` diberlakukan merata untuk sel $(0, 0)$ hingga $(100, 100)$.
  - *Koreksi Realitas*: Iklim wajib bervariasi secara spasial berdasarkan **Lintang (*Latitude*)** dan **Elevasi (*Altitude*)**:
    1. **Zona Tropis / Khatulistiwa ($0^\circ - 15^\circ$ Lat)**:
       - Tidak ada 4 musim. Suhu rata-rata konstan hangat ($T > 18^\circ\text{C}$).
       - Siklus iklim adalah **Monsun Hujan (*Wet*)** dan **Kemarau (*Dry*)**.
       - Pertumbuhan biomassa terjadi sepanjang tahun, bukan mati di musim dingin.
    2. **Zona Subtropis / Gurun ($15^\circ - 30^\circ$ Lat)**:
       - Presipitasi sangat rendah, kelembapan rendah, fluktuasi suhu diurnal ekstrem (siang panas membakar, malam sangat dingin).
    3. **Zona Sedang (*Temperate*) ($30^\circ - 55^\circ$ Lat)**:
       - Memiliki 4 musim sejati (*Spring, Summer, Autumn, Winter*) dengan fluktuasi panjang siang hari (*photoperiod*).
    4. **Zona Subpolar / Kutub ($> 60^\circ$ Lat)**:
       - Musim dingin dominan/panjang, tanah permafrost, tundra, pertumbuhan vegetasi sangat singkat (1-2 bulan).

### Dimensi 2: Topografi, Elevasi & Laju Penurunan Suhu (Lapse Rate)
- [ ] **Audit Pengaruh Ketinggian**:
  - *Hukum Fisika*: Suhu udara turun secara kontinu seiring bertambahnya ketinggian mengikuti **Environmental Lapse Rate** ($\approx -6.5^\circ\text{C}$ per kenaikan 1,000 meter).
  - *Verifikasi Code*: Gunung tinggi di daerah khatulistiwa (misal puncak salju Andes atau Puncak Jaya) harus bersuhu dingin meskipun terletak di lintang tropis.
- [ ] **Efek Bayangan Hujan (*Orographic Rain Shadow*)**:
  - Angin lembap yang menabrak pegunungan menghasilkan hujan lebat di lereng hadap-angin (*windward*), sedangkan lereng sebaliknya (*leeward*) menjadi daerah kering/semi-arid.

### Dimensi 3: Termodinamika, Alometri & Metabolisme Biologis
- [ ] **Audit Skala Metabolisme (Kleiber's Law)**:
  - Laju metabolisme basal ($BMR$) makhluk hidup berskala alometrik terhadap massa tubuh: $BMR \propto \text{Mass}^{0.75}$.
- [ ] **Biaya Termoregulasi Lingkungan**:
  - Di lingkungan dingin ekstrem ($< 5^\circ\text{C}$): Manusia membakar kalori ekstra untuk mempertahankan homeotermia suhu inti tubuh ($37^\circ\text{C}$), menuntut pakaian tebal, tempat berteduh, atau api unggun.
  - Di lingkungan panas ekstrem ($> 35^\circ\text{C}$): Kebutuhan cairan/air meningkat drastis; tanpa sumber air tawar, terjadi dehidrasi cepat.
- [ ] **Hukum Minimum Liebig (*Liebig's Law of the Minimum*)**:
  - Pertumbuhan populasi dan kelangsungan hidup tidak ditentukan oleh total sumber daya yang berlimpah, melainkan oleh **unsur esensial yang paling langka** (misal: kalori melimpah tapi tanpa garam/mineral $\to$ defisiensi elektrolit; atau makanan banyak tapi tanpa air tawar $\to$ mati dehidrasi).

### Dimensi 4: Rantai Trofik & Daya Dukung Lingkungan (*Carrying Capacity*)
- [ ] **Audit Konservasi Biomassa & Efisiensi Lindeman**:
  - Efisiensi perpindahan energi antar-tingkat trofik hanya $\sim 10\%$ (Produsen Primer $\to$ Herbivora $\to$ Karnivora/Manusia).
  - Biomassa predator/manusia tidak boleh melampaui batas energi yang dihasilkan fotosintesis produsen primer di wilayah jelajah mereka.
- [ ] **Kinetika Pemulihan Logistik**:
  - Sumber daya hayati (hutan, ikan, hewan buruan) pulih mengikuti kurva logistik: lambat saat hampir punah ($M < 0.05$), cepat di fase tengah, dan melambat saat mendekati kapasitas tampung lingkungan ($K$).

### Dimensi 5: Demografi, Usia & Fisiologi Reproduksi Manusia
- [ ] **Audit Parameter Kehidupan Manusia**:
  - *Mortalitas Gompertz-Makeham*: Risiko kematian meningkat secara eksponensial seiring bertambahnya usia biologis.
  - *Jendela Reproduksi Alami*: Fertilitas manusia terbatas pada usia biologis $\sim 15 - 45$ tahun.
  - *Biaya Energetik Kehamilan & Laktasi*: Ibu hamil dan menyusui membutuhkan asupan kalori 20-30% lebih tinggi. Kegagalan pasokan kalori memicu penurunan fertilitas alami (*amenorrhea* akibat malnutrisi).
  - *Rasio Gender Alami*: Rasio jenis kelamin saat lahir stabil secara biologis pada kisaran $\sim 105$ bayi laki-laki per 100 bayi perempuan.

### Dimensi 6: Validasi Pattern-Oriented Modeling (POM) & Protokol ODD
- [ ] **Audit Validasi Struktural (Volker Grimm / Science 2005)**:
  - Model tidak boleh divalidasi hanya dengan mencocokkan satu angka agregat acak.
  - Model dianggap valid secara realitas jika aturan mikronya **mampu mereproduksi minimal 3 pola makro empiris secara simultan**:
    1. *Pola Demografi*: Piramida usia berbentuk segitiga stabil (banyak usia muda, sedikit lansia).
    2. *Pola Spasial*: Terkonsentrasinya pemukiman di sepanjang lembah sungai dan sumber air tawar.
    3. *Pola Pasar*: Terbentuknya gradien harga di mana komoditas langka bernilai lebih tinggi di wilayah yang tidak memiliki akses langsung.

### Dimensi 7: Riset Internet Empiris Berbasis Waktu Terkini (search_web [Bulan] [Tahun])
- [ ] **Kewajiban Validasi Riset Terkini**:
  - Untuk setiap fenomena biofisik, ekologis, atau demografis yang terdeteksi anomali atau belum optimal, agen **WAJIB MELAKUKAN PENELUSURAN INTERNET** via `search_web` dengan menyematkan Bulan dan Tahun berjalan (misal: `"October 2026"`, `"2026"`).
  - Mengintegrasikan model matematika dan literatur ilmiah mutakhir (seperti persamaan logistik pemanenan periodik non-otonom dengan jendela pemijahan, kurva peluruhan pangan empiris forager, atau formula carrying capacity multi-spesies).
  - Menghindari asumsi spekulatif tanpa rujukan ilmiah terindeks.

### Dimensi 8: Kepatuhan Realitas Rantai Pasok Resep & Keragaman Hasil Buruan (Hunting Diversity & Manufacturing Supply Chain Reality)
- [ ] **Audit Asal Usul Item (Alam vs Transformasi Resep Manufaktur)**:
  - Setiap komoditas di dalam simulasi wajib diaudit secara ontologis: *apakah masuk akal item ini dipetik langsung dari alam, ataukah wajib berasal dari transformasi kerja (resep)?*
  - **Larangan Bahan Tidak Logis**:
    - *Alat Batu Tanpa Batu*: Dilarang memodelkan kapak batu (`STONE_AXE`) atau sabit batu murni dari kayu tanpa komponen batu (`STONE` / `FLINT`).
    - *Pangan Mentah yang Tidak Dapat Dicerna*: Dilarang membiarkan biji serealia mentah (`GRAIN`) dikonsumsi langsung tanpa proses penggilingan (*milling*) menjadi tepung atau pemanggangan menjadi roti/bubur.
    - *Anomali Asal Ganda*: Dilarang merancukan tanaman liar mentah dengan ramuan obat jadi siap pakai (`HERBAL_MEDICINE`).
  - **Keragaman Fauna & Hasil Perburuan (*Wild Game Hunting Diversity*)**:
    - Alam tidak boleh hanya menyediakan satu jenis fauna air tawar (`FISH`).
    - Habitat darat (hutan, perbukitan, padang rumput) wajib memiliki simpul perburuan fauna darat (*cervids, wild boar, ungulates*) yang menghasilkan trias produk perburuan arkeologis:
      1. **Daging Mentah Segar (`RAW_MEAT`)**: Rentan busuk (2–3 hari), membutuhkan rantai pasok pengasapan/pengeringan (*Smoked/Dried Meat*).
      2. **Kulit Binatang Mentah (`RAW_HIDE`)**: Bahan baku vital pembuatan pakaian pelindung dingin.
      3. **Barang Modal Pelindung Tubuh (`LEATHER_CLOTHING`)**: Wajib dibuat melalui resep penyamakan dan penjahitan kulit untuk memitigasi bahaya mortalitas hipotermia cuaca dingin.

### Dimensi 9: Mobilitas Spasial, Dinamika Sel Peta & Akses Lintas Laut (Spatial Mobility & Island Exploration)
- [ ] **Audit Keterikatan Koordinat Statis (*Static Settlement Trap*)**:
  - *Red Flag*: Seluruh populasi lahir, hidup, dan mati di satu sel tunggal (misal `(15, 25)`), sementara 99% peta dunia tidak pernah dijelajahi.
  - *Koreksi Realitas*: Agen manusia pemburu-peramu dan petani purba memiliki mobilitas spasial:
    1. Berpindah sel menuju simpul foraging yang paling optimal (*Charnov marginal foraging path*).
    2. Menjelajah sel baru saat sumber daya lokal menipis atau populasi meningkat (*territorial expansion*).
- [ ] **Audit Aksesibilitas Simpul Ekologis Terpencil**:
  - *Red Flag*: Simpul vital (seperti tambang garam atau deposit mineral pulau) berada di koordinat terpencil (jarak >30 sel) yang terputus dari radius jangkauan foraging harian ($d \le 5.0$).
  - *Koreksi Realitas*: Membutuhkan ekspedisi maritim terencana dengan perkakas modal air fisik (`RAFT`), di mana agen berlayar menembus sel `DeepOcean` untuk mengekstrak komoditas pulau dan membawanya pulang ke pemukiman induk.

### Dimensi 10: Rantai Nilai Pangan Neolitik & Repertoar Teknologi 1.000 Tahun (The 1,000-Year Neolithic Package)
- [ ] **Audit Rantai Pengolahan Gandum & Makanan Pokok**:
  - *Hukum Nutrisi Arkeologis*: Manusia tidak memiliki rumen atau sistem pencernaan burung untuk mencerna biji serealia mentah (`RAW_GRAIN`) utuh.
  - *Rantai Wajib*:
    1. Sabit pemanen mikrolit (`HARVESTING_SICKLE`).
    2. Penggilingan mekanik via batu gilang/lesung (`SADDLE_QUERN` / `MORTAR_AND_PESTLE`).
    3. Tepung gandum halus (`GRAIN_FLOUR`).
    4. Pengolahan termal api menjadi roti pipih bakar (`FLATBREAD`) atau bubur rebus (`PORRIDGE`).
- [ ] **Audit Evolusi Material 36 Generasi (1.000 Tahun)**:
  - Dalam rentang 1.000 tahun (transisi Mesolitik akhir ke Neolitik Penuh dan Kalkolitik), peradaban manusia wajib membuka:
    - *Piroteknologi Arang*: `CHARCOAL` ($>1.000^\circ\text{C}$) dari pirolisis kayu untuk pembakaran tembikar kedap dan peleburan tembaga.
    - *Tekstil Nabati*: Serat pintal (`CORDAGE`), pemintal benang (`SPINDLE_WHORL`), dan tenun (`WOVEN_TEXTILE`).
    - *Senjata Berburu Akurat*: Busur & anak panah (`BOW_AND_ARROW`) dan jerat hewan (`ANIMAL_SNARE`).
    - *Metalurgi Tembaga Awal*: Ekstraksi bijih malakit/tembaga (`COPPER_ORE`) $\to$ batangan (`COPPER_INGOT`) $\to$ bilah pahat (`COPPER_BLADE`).

### Dimensi 11: Difusi Pengetahuan, Garansi Anti-Deadlock Eureka & Transmisi Kultural
- [ ] **Audit Kebuntuan Prasyarat Telur-Ayam (*Chicken-and-Egg Eureka Deadlock*)**:
  - *Red Flag*: Pengetahuan $K$ mensyaratkan memegang item $X$, namun item $X$ hanya bisa didapatkan atau diolah jika agen sudah menguasai pengetahuan $K$ atau sarana yang belum ditemukan.
  - *Contoh Nyata*: `KNOWLEDGE_FISH_CURING` membutuhkan `SALT`, tetapi garam berada di pulau terpencil yang membutuhkan perahu rakit dan pengawetan untuk ekspedisi.
- [ ] **Audit Konkurensi Bahan Simultan yang Ekstrem**:
  - *Red Flag*: Eureka mensyaratkan memegang 3 bahan rival sekaligus (misal Clay + Timber + Fire) pada satu tick yang sama, sementara agen selalu mengonsumsi salah satu bahan untuk kebutuhan bertahan hidup darurat.
- [ ] **Audit Kelengkapan Kurikulum Magang (*Apprenticeship Curriculum Completeness*)**:
  - *Red Flag*: Suatu cetak biru pengetahuan tidak terdaftar dalam daftar tukar magang `trade.rs`, sehingga ketika penemu tunggal wafat tanpa anak hidup, pengetahuan tersebut punah selamanya dari peradaban (*technological extinction trap*).
- [ ] **Audit Transmisi Kultural Antargenerasi (*Parental Tutoring*)**:
  - Anak usia remaja (12–18 tahun) secara alami menyerap keahlian orang tuanya sebelum orang tua wafat, menjamin transmisi budaya kumulatif (*ratchet effect*) tanpa kehilangan memori kolektif.

---

## 🛠️ Checklist Praktis Saat Mengaudit Kode Spasial & Lingkungan di `src/`

Gunakan checklist ini saat meninjau modul geografi dan iklim:

| Komponen di Code | Asumsi Naif (Wajib Ditolak) | Realitas Ilmiah (Solusi Benar) |
| :--- | :--- | :--- |
| **Iklim Global** | Seluruh peta mengalami salju di bulan Desember. | Lintang $0^\circ$ (khatulistiwa) mengalami monsun basah/kering tanpa salju. |
| **Suhu Sel** | Suhu sama rata di semua sel daratan. | Suhu dihitung dari: $\text{BaseTemp}(\text{Latitude}) - 0.0065 \times \text{Elevation}$. |
| **Konsumsi Air** | Agen hanya butuh kalori makanan padat. | Agen butuh akses ke sel air tawar (*River/Lake*) untuk mencegah dehidrasi. |
| **Regenerasi Hutan** | Hutan pulih instan dalam hitungan tick. | Hutan purba butuh puluhan tahun untuk mencapai biomassa kayu penuh (*Slow pace*). |
| **Transportasi Air** | Berjalan kaki menembus danau/laut. | Membutuhkan perkakas modal transportasi air fisik (`RAFT`) dengan biaya energi. |
| **Kematian** | Probabilitas mati acak merata semua usia. | Mortalitas mengikuti kurva Gompertz-Makeham (tinggi di bayi & usia lanjut). |
| **Bahan Baku Alat** | Kapak batu dibuat 100% dari kayu balok. | Kapak batu membutuhkan komponen batuan mineral keras (`STONE`) + gagang kayu. |
| **Fauna & Buruan** | Protein hewani hanya ikan sungai. | Keragaman fauna darat perbukitan/hutan menghasilkan daging mentah dan kulit binatang. |
| **Ketahanan Dingin** | Menahan dingin hanya dengan kayu bakar. | Pakaian kulit jahitan (`LEATHER_CLOTHING`) sebagai perisai termal tubuh. |

---

*Dengan menerapkan audit realitas biofisik ini, simulator ekonomi tidak akan terjebak dalam ilusi simulasi eurosentris atau kartun buatan, melainkan mencerminkan kendala geografis, faunal, dan ekologis nyata bumi.*
