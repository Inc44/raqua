// path/filename: recover_files.rs

use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::path::Path;
use std::time::Instant;

const SECTOR_SIZE: usize = 4096;
const SEARCHED_CHUNK_SIZE: usize = 16;

fn main() -> io::Result<()> {
    let file_path = Path::new(".bin");
    //let file_path = Path::new("C:/Users/PC/Desktop/bench.bin");
    let save_path = Path::new("/recovered");

    if save_path.exists() {
        fs::remove_dir_all(save_path)?;
    }
    fs::create_dir_all(save_path)?;

    let start_time = Instant::now();
    let mut file = OpenOptions::new().read(true).open(file_path)?;
    let file_size = file.seek(SeekFrom::End(0))?;
    file.seek(SeekFrom::Start(0))?;

    let number_of_sectors = file_size as usize / SECTOR_SIZE;
    let mut sector_number: usize = 1;

    while sector_number <= number_of_sectors {
        let start_sector = sector_number;
        let mut chunk = Vec::with_capacity(SECTOR_SIZE * SEARCHED_CHUNK_SIZE);

        loop {
            let mut sector = vec![0u8; SECTOR_SIZE];
            file.read_exact(&mut sector)?;
            chunk.extend_from_slice(&sector);
            sector_number += 1;

            if is_end_of_chunk(&sector) || sector_number > number_of_sectors {
                break;
            }
        }

        let end_sector = sector_number;
        let chunk_size = end_sector - start_sector;

        if chunk_size <= SEARCHED_CHUNK_SIZE
            && !is_null_bytes_only(&chunk)
            && is_printable_or_null(&chunk)
        {
            let chunk_file_path = save_path.join(format!("{}.{}.bin", start_sector, end_sector));
            let mut chunk_file = File::create(chunk_file_path)?;
            chunk_file.write_all(&chunk)?;
        }
    }

    let end_time = Instant::now();
    println!("Execution Time: {:?}", end_time.duration_since(start_time));

    Ok(())
}

fn is_end_of_chunk(sector: &[u8]) -> bool {
    let first_null_byte_index = sector.iter().position(|&x| x == 0x00);
    match first_null_byte_index {
        Some(index) => sector[index..].iter().all(|&x| x == 0x00),
        None => false,
    }
}

fn is_null_bytes_only(chunk: &[u8]) -> bool {
    chunk.iter().all(|&x| x == 0x00)
}

fn is_printable_or_null(chunk: &[u8]) -> bool {
    chunk
        .iter()
        .all(|&x| x == 0x00 || x >= 0x20 || x == 0x09 || x == 0x0A || x == 0x0D)
}
