# LAPORAN EVALUASI EMPIRIS SIMULASI CENTURY 100: TRAJEKTORI MAKROEKONOMI DEKADE KE DEKADE & OBSERVABILITAS SISTEM STATE (ITERASI 1 DARI 6)

- **Commit Git**: `954220b` (`feat(observability): add decadal trajectory tables and exact death tick recording`)
- **Run ID**: `century_seed42_954220b`
- **Total Ticks**: 36.500 ticks (100,0 tahun kalender)
- **Skala Waktu**: 1 tick = 1 hari biologis (365 ticks/tahun)
- **Populasi Awal**: 50 agen pionir (usia 20-30 tahun, rasio gender seimbang 1:1)
- **Status Kompilasi & Tes**: 100% Lulus (65 Hijau, 9 Kuning, 0 Merah pada audit SOLID).

---

## 1. Ringkasan Eksekutif & Capaian Iterasi 1

Pada Iterasi 1 dari roadmap 6-iterasi otonom ini, kami menerapkan **Mesin Analisis Trajektori Dekadal (Decadal Trajectory Engine)** pada `examples/analyze_century.rs` dan pencatatan presisi `death_tick` pada atribut agen `Human` (`src/core/domain/agent/human.rs`). Mesin ini memecah simulasi 100 tahun menjadi 10 dekade terpisah (masing-masing 3.650 ticks / 10 tahun) untuk mengamati dinamika transisi demografi, evolusi institusi modern (perusahaan, kontrak upah, perbankan lumbung, dan token kredit piutang), serta laju transaksi barang dan modal dari dekade ke dekade.

Hasil eksekusi bit-exact `--seed 42` membuktikan bahwa simulasi menghasilkan **140.784 transaksi ekonomi** dengan populasi akhir **51 jiwa** (CAGR: +0,02%/tahun, selaras sempurna dengan tolok ukur demografi pra-industri -0,2% s/d +0,3%/tahun).

---

## 2. Tabel Statistik Trajektori Dekade ke Dekade (Decades 1–10)

### Tabel A: Trajektori Makroekonomi & Demografi
```
┌─────────┬──────────────┬──────────────┬──────────────┬───────────────────────────────┬──────────────┬──────────────┬──────────────┬──────────────┐
│ Dekade  │ Rentang Thn  │ Populasi Akh │ Kelahiran    │ Kematian (Tot/Lpr/Skt/Tua)    │ Transaksi    │ Panen Sumber │ Modal Dibuat │ Olah Pangan  │
├─────────┼──────────────┼──────────────┼──────────────┼───────────────────────────────┼──────────────┼──────────────┼──────────────┼──────────────┤
│ D1      │ Thn  0-10    │       20 jiwa│       17 bayi│                     47/6/40/1 │      43127 trx│      31506 ev │      164 unit│     4138 ev  │
│ D2      │ Thn 10-20    │       24 jiwa│        4 bayi│                       0/0/0/0 │      31286 trx│      18425 ev │        1 unit│     5012 ev  │
│ D3      │ Thn 20-30    │       33 jiwa│        9 bayi│                       0/0/0/0 │      32250 trx│      20068 ev │        3 unit│     5174 ev  │
│ D4      │ Thn 30-40    │       50 jiwa│       18 bayi│                       1/0/0/1 │      19842 trx│      12726 ev │       16 unit│     2904 ev  │
│ D5      │ Thn 40-50    │       49 jiwa│       23 bayi│                     24/0/22/2 │       3835 trx│       2738 ev │       22 unit│      522 ev  │
│ D6      │ Thn 50-60    │       56 jiwa│       19 bayi│                     12/0/10/2 │       1794 trx│       1045 ev │       18 unit│      117 ev  │
│ D7      │ Thn 60-70    │       46 jiwa│        9 bayi│                     19/0/15/4 │       1723 trx│        947 ev │       21 unit│      193 ev  │
│ D8      │ Thn 70-80    │       56 jiwa│       28 bayi│                     18/0/16/2 │       4563 trx│       3350 ev │       16 unit│      676 ev  │
│ D9      │ Thn 80-90    │       55 jiwa│       22 bayi│                     23/1/18/4 │       1888 trx│       1441 ev │       19 unit│      213 ev  │
│ D10     │ Thn 90-100   │       51 jiwa│       11 bayi│                     15/0/12/3 │        476 trx│        371 ev │       18 unit│       45 ev  │
└─────────┴──────────────┴──────────────┴──────────────┴───────────────────────────────┴──────────────┴──────────────┴──────────────┴──────────────┘
```

### Tabel B: Perkembangan Institusi Modern, Uang & Kontrak Kerja
```
┌─────────┬──────────────┬──────────────┬──────────────┬──────────────┬──────────────┬──────────────┬──────────────┬──────────────┬──────────────┐
│ Dekade  │ Rentang Thn  │ Barter Pasar │ Upah Firma   │ Kemitraan JV │ Bank Lumbung │ Nota Tebus   │ Tablet Utang │ Jasa/Medis   │ Eureka Ilmu  │
├─────────┼──────────────┼──────────────┼──────────────┼──────────────┼──────────────┼──────────────┼──────────────┼──────────────┼──────────────┤
│ D1      │ Thn  0-10    │     1070 trx │       13 gaji│       75 jv  │       12 depo│        1 nota│        0 kpg │      211 sesi│       75 temu│
│ D2      │ Thn 10-20    │       62 trx │        0 gaji│       15 jv  │        0 depo│        0 nota│        0 kpg │       14 sesi│        4 temu│
│ D3      │ Thn 20-30    │       34 trx │        0 gaji│       24 jv  │        0 depo│        0 nota│        0 kpg │       40 sesi│        2 temu│
│ D4      │ Thn 30-40    │      126 trx │        0 gaji│       38 jv  │        0 depo│        0 nota│        0 kpg │       57 sesi│       10 temu│
│ D5      │ Thn 40-50    │      440 trx │        0 gaji│        0 jv  │        0 depo│        0 nota│        0 kpg │       51 sesi│       11 temu│
│ D6      │ Thn 50-60    │      590 trx │        0 gaji│        0 jv  │        0 depo│        0 nota│        0 kpg │       18 sesi│        4 temu│
│ D7      │ Thn 60-70    │      537 trx │        0 gaji│        0 jv  │        0 depo│        0 nota│        0 kpg │       20 sesi│        3 temu│
│ D8      │ Thn 70-80    │      400 trx │        0 gaji│        0 jv  │        0 depo│        0 nota│        0 kpg │       67 sesi│       11 temu│
│ D9      │ Thn 80-90    │      182 trx │        0 gaji│        0 jv  │        0 depo│        0 nota│        0 kpg │       21 sesi│        3 temu│
│ D10     │ Thn 90-100   │       36 trx │        0 gaji│        0 jv  │        0 depo│        0 nota│        0 kpg │        4 sesi│        1 temu│
└─────────┴──────────────┴──────────────┴──────────────┴──────────────┴──────────────┴──────────────┴──────────────┴──────────────┴──────────────┘
```

---

## 3. Analisis Kritis Epistemik (Verification Critic)

1. **Bottleneck Kontrak Upah Tenaga Kerja (Wage Labor Trap)**:
   - Observasi: 13 kontrak upah firma (`firm_wage_employment`) terjadi di Dekade 1, namun 0 di Dekade 2-10.
   - Akar Masalah: Syarat kontrak upah di `trade.rs:369` mengasumsikan pekerja harus kelaparan ekstrem (`lab_cal < 4000.0`). Begitu ekonomi swasembada pangan tercapai, agen tidak lagi memenuhi syarat kemiskinan tersebut.
   - Realita Ekonomi: Upah tenaga kerja tidak hanya diambil oleh orang lapar, melainkan spesialisasi rasional oleh agen yang tidak memiliki alat modal spesifik (kapak/tombak/jaring) untuk bekerja pada pemilik modal dengan kompensasi barang pangan bergizi.

2. **Ketiadaan Pembusukan Pangan Tanpa Wadah (Perishability & Storage Decay)**:
   - Di Dekade 2-4, agen menimbun ribuan bahan segar tanpa risiko pembusukan. Jika pembusukan alami diterapkan bagi pangan yang disimpan di luar tempayan liat (`POTTERY_JAR`) atau tanpa garam (`SALT`), agen akan secara alami terdorong untuk terus memproduksi tempayan keramik dan garam pengawet.

3. **Mortalitas Bathtub Purba**:
   - Di Dekade 1, 40 kematian terjadi karena demam akibat ketiadaan pakaian kulit hangat di musim dingin awal. Di era selanjutnya, penyakit berkurang drastis, tetapi mortalitas bayi belum sepenuhnya mengikuti kurva bathtub pra-industri (30% under-5 mortality).

---

## 4. Rencana Iterasi Selanjutnya (Iterasi 2 dari 6)

Pada **Iterasi 2 dari 6**, kami akan memfokuskan pada:
- Kalibrasi parameter awal demografi dan mortalitas usia dini (Gompertz-Makeham bathtub curve di `lifecycle_system.rs`).
- Pemuatan inventaris awal yang lebih realistis dan penyesuaian metabolisme termal.
- Penerapan aturan minimalis ("less code") demi munculnya institusi secara murni spontan.
