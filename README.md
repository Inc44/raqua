# Introduction

Raqua 💧, a set of Python scripts and Rust program, is designed to scan an ocean of disk copies, such as `.bin` and retrieve files lacking conventional signatures, such as `.txt`, `.md`, `.tex`, by creating an overflowing cache for `grep`.

# Motivation

One day, after using `Obsidian` for around 8 hours to study informatics and philosophy, I finished a substantial amount of work. I decided to sync my changes to `Google Drive` using `rclone`. I initiated an `Upload Task` and, mistakenly believing it had completed due to no indication in the `Rclone Browser`, I started the `Download Task`. Unfortunately, the upload hadn't finished, resulting in the loss of 8 hours of work across approximately 16 files, equating to 64 KiB of markdown text. Only one file had been successfully uploaded. In a state of shock and panic, I considered shutting down my PC to prevent data from being overwritten. Just before it turned off, the realization hit me that I could use `Ctrl + Z` in `Obsidian` to at least recover the modified files (about 48 KiB of markdown text), if not the newly created ones. However, attempting to cancel the shutdown was futile, as `shutdown -c` did not work. The emotional distress was overwhelming when I acknowledged the magnitude of my mistake. Realizing my last `Notes` commit was several days prior, I knew that relying on `git` was futile. Thus, I decided to boot from my Ventoy recovery USB with `Hiren’s BootCD PE` to attempt file recovery. Despite trying `PhotoRec` (`TestDisk`), `Recuva`, and `DMDE`, none managed to recover the .md files. Consequently, I created a sector-by-sector .bin file of the disk. Even after attempting to use `Hetman Partition Recovery`, which deemed them corrupted after 6 hours of analysis, I managed to locate some fragments of the missing text using `HxD Editor`. However, the realization that searching through 100 GiB would take an impractical amount of time led me to ponder a more efficient solution. Hence, I devised a Python script that scans the disk's bit-by-bit copy sector by sector, creating a cache for `grep` (`winGrep`).

# Notes

After recovering my files, I discovered [`RecoverPy`](https://github.com/PabloLec/RecoverPy) on GitHub and highly recommend it for future use due to its user-friendly approach. This experience, albeit frustrating, was immensely educational, offering me deep insights into files, their structures and filesystems.

# Installation

To install, download the Python script to your local machine and verify that Python is installed to run the script, as it represents the sole external dependency. For Rust, use `git clone https://github.com/Inc44/raqua.git`, navigate with `cd raqua`, and execute `cargo run --release`.

# Explanation/Pseudo Code

## Goal
Recover data from a bit-to-bit copy of a 100GiB ext4 partition with 4K sectors.

## Why It Works
Based on the probability calculation, files are typically adjacent to each other and don't occupy a full sector.

$ \lim \left(\frac{1}{2}\right)^{8\times\text{length of consecutive NULLs}}\times100\times1024^3 = 0 $

## Process Overview

1. Initialize variables:
   - file_path = Path to the binary file representing the bit-to-bit copy
   - save_path = Directory where recovered files will be saved
   - sector_size = Size of a sector in bytes (4096 bytes)
   - searched_chunk_size = Maximum size of a file to search for in sectors (16 sectors)

2. Prepare the save directory:
   - If save_path exists, remove it and its contents
   - Create a new directory at save_path

3. Define helper functions:
   - is_end_of_chunk(sector): Determines if the sector marks the end of a chunk based on trailing null bytes
   - is_null_bytes_only(chunk): Checks if a chunk consists only of null bytes
   - is_printable_or_null(chunk): Verifies if a chunk contains only printable or null bytes

4. Open the binary file for reading in binary mode

5. Calculate the number of sectors in the file

6. Iterate over each sector of the file:
   - Read sectors one by one
   - Accumulate consecutive sectors into a chunk until an end-of-chunk condition is met or the file ends

7. For each chunk identified:
   - If the chunk is smaller than or equal to the searched_chunk_size, does not consist only of null bytes, and contains only printable or null bytes:
     - Save the chunk to a file in the save_path directory, naming the file with the starting and ending sector numbers

# License

MIT