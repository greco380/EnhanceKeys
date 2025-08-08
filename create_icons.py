#!/usr/bin/env python3
from PIL import Image, ImageDraw, ImageFont
import os

def create_icon(size, filename):
    # Create a new image with blue background
    img = Image.new('RGBA', (size, size), (70, 130, 180, 255))
    draw = ImageDraw.Draw(img)
    
    # Add white text "EK" in the center
    try:
        # Try to use a system font
        font = ImageFont.truetype("/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf", size//3)
    except:
        # Fallback to default font
        font = ImageFont.load_default()
    
    # Calculate text position to center it
    text = "EK"
    bbox = draw.textbbox((0, 0), text, font=font)
    text_width = bbox[2] - bbox[0]
    text_height = bbox[3] - bbox[1]
    
    x = (size - text_width) // 2
    y = (size - text_height) // 2
    
    draw.text((x, y), text, fill=(255, 255, 255, 255), font=font)
    
    # Save the image
    img.save(filename, 'PNG')
    print(f"Created {filename}")

# Create icons directory
os.makedirs('/root/EnhanceKeys/EnhanceKeys/src-tauri/icons', exist_ok=True)
os.chdir('/root/EnhanceKeys/EnhanceKeys/src-tauri/icons')

# Create required PNG icons
create_icon(32, '32x32.png')
create_icon(128, '128x128.png') 
create_icon(256, '128x128@2x.png')  # 2x version is typically double size

print("Icon creation completed!")