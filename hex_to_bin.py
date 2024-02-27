import binascii

hex_file_path = ".hex"
binary_file_path = ".bin"
with open(hex_file_path, "r") as hex_file:
    hex_data = hex_file.read().strip()
    # hex_data = hex_file.read(2*4096*32).strip()
    hex_data = hex_data.replace(" ", "")
binary_data = binascii.unhexlify(hex_data)
with open(binary_file_path, "wb") as bin_file:
    bin_file.write(binary_data)
