#!/usr/bin/env python3
import struct
import zlib

def create_png(width, height, rgb_color, filename):
    """Create a simple solid color PNG file without PIL"""
    
    # PNG signature
    png_signature = b'\x89PNG\r\n\x1a\n'
    
    # IHDR chunk
    ihdr_data = struct.pack('>IIBBBBB', width, height, 8, 2, 0, 0, 0)  # RGB, no alpha
    ihdr_crc = zlib.crc32(b'IHDR' + ihdr_data) & 0xffffffff
    ihdr_chunk = struct.pack('>I', 13) + b'IHDR' + ihdr_data + struct.pack('>I', ihdr_crc)
    
    # Create image data (solid color)
    scanlines = []
    for y in range(height):
        scanline = b'\x00'  # Filter type 0 (None)
        for x in range(width):
            scanline += struct.pack('BBB', rgb_color[0], rgb_color[1], rgb_color[2])
        scanlines.append(scanline)
    
    # Compress image data
    raw_data = b''.join(scanlines)
    compressed_data = zlib.compress(raw_data)
    
    # IDAT chunk
    idat_crc = zlib.crc32(b'IDAT' + compressed_data) & 0xffffffff
    idat_chunk = struct.pack('>I', len(compressed_data)) + b'IDAT' + compressed_data + struct.pack('>I', idat_crc)
    
    # IEND chunk
    iend_crc = zlib.crc32(b'IEND') & 0xffffffff
    iend_chunk = struct.pack('>I', 0) + b'IEND' + struct.pack('>I', iend_crc)
    
    # Write PNG file
    with open(filename, 'wb') as f:
        f.write(png_signature + ihdr_chunk + idat_chunk + iend_chunk)
    
    print(f"Created {filename} - {width}x{height} RGB{rgb_color}")

# Change to icons directory
import os
os.makedirs('/root/EnhanceKeys/EnhanceKeys/src-tauri/icons', exist_ok=True)
os.chdir('/root/EnhanceKeys/EnhanceKeys/src-tauri/icons')

# Create solid color PNG icons (blue theme)
create_png(32, 32, (70, 130, 180), '32x32.png')          # Steel blue
create_png(128, 128, (70, 130, 180), '128x128.png')      # Steel blue
create_png(256, 256, (70, 130, 180), '128x128@2x.png')   # Steel blue @2x

print("Simple PNG icon creation completed!")