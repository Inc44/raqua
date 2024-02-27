# goal : recover files of size less than 64kib from bit-to-bit copy of 100gib ext4 partition with 4k sectors
# why_it_works : files are next to each other, we consider files not occupying full sectors as (1/2)^8)^64 * 100*1024^3 < 10^-100
import time
import os
import shutil

file_path = ".bin"
save_path = "/recovered"
sector_size = 4096
searched_chunk_size = 16
if os.path.exists(save_path):
    shutil.rmtree(save_path)
os.makedirs(save_path)


# consider end of chunk if null bytes continue until end of sector from first null byte
def is_end_of_chunk(sector):
    first_null_byte_index = sector.find(b"\x00")
    if first_null_byte_index == -1:
        return False
    return sector[first_null_byte_index:] == b"\x00" * (
        len(sector) - first_null_byte_index
    )


def is_null_bytes_only(chunk):
    return chunk == b"\x00" * len(chunk)


def is_printable_or_null(chunk):
    for byte in chunk:
        if byte not in (0x00, 0x09, 0x0A, 0x0D) and (byte < 0x20):
            return False
    return True


start_time = time.perf_counter()
with open(file_path, "rb") as file:  # read only, binary mode
    file.seek(0, os.SEEK_END)
    file_size = file.tell()
    file.seek(0)
    number_of_sectors = file_size / sector_size
    sector_number = 1
    while sector_number <= number_of_sectors:
        start_sector = sector_number
        sector = file.read(sector_size)
        chunk = sector
        while not is_end_of_chunk(sector) and sector_number <= number_of_sectors:
            sector_number += 1
            sector = file.read(sector_size)
            chunk += sector
        end_sector = sector_number
        chunk_size = end_sector - start_sector
        if (
            chunk_size <= searched_chunk_size
            and not is_null_bytes_only(chunk)
            and is_printable_or_null(chunk)
        ):
            with open(
                f"{save_path}/{start_sector}.{end_sector}.bin", "wb"
            ) as chunk_file:
                chunk_file.write(chunk)
end_time = time.perf_counter()
execution_time = end_time - start_time
print(execution_time)
