use chrono::prelude::Local;

use crate::model::kegiatan::KegiatanTambah;
use crate::model::Record;
use crate::store::persistent::DB;

#[tauri::command]
pub async fn tambahkegiatan(name: String) -> Result<String, String> {
    let created: Result<Vec<Record>, surrealdb::Error> = DB
        .create("kegiatan")
        .content(KegiatanTambah {
            name,
            create: Local::now(),
            update: Local::now(),
        })
        .await;

    match created {
        Ok(data) => {
            let kegiatan = data.first();
            if let Some(keg) = kegiatan {
                Ok(format!("Kegiatan #{} berhasil disimpan.", keg.id))
            } else {
                Ok("ID kegiatan tidak ditemukan saat penimpanan.".to_string())
            }
        }

        Err(_) => {
            let msg = "Kegiatan belum berhasil disimpan.".to_string();

            Err(msg)
        }
    }
}