#!/bin/bash

# 参数检查
if [ $# -ne 2 ]; then
    echo "Usage: $0 <source_png> <destination_dir>"
    exit 1
fi

# 输入参数
input="$1"
dest_dir="$2"

# 检查源文件是否存在
if [ ! -f "$input" ]; then
    echo "Error: Source file '$input' does not exist."
    exit 1
fi

# 验证文件类型是否为PNG
if [[ $(file -b --mime-type "$input") != "image/png" ]]; then
    echo "Error: Source file must be a PNG image."
    exit 1
fi

# 处理目标目录
if [ ! -d "$dest_dir" ]; then
    echo "Warning: Destination directory '$dest_dir' does not exist."
    # 使用脚本执行目录作为基础路径
    script_dir=$(dirname "$(realpath "$0")")
    dest_dir="$script_dir/$dest_dir"
    echo "Creating destination in script directory: $dest_dir"
fi

# 创建目标目录
mkdir -p "$dest_dir" || exit 1

# 提取基础文件名
filename=$(basename -- "$input")
basename="${filename%.*}"

# 创建临时图标集目录
temp_dir=$(mktemp -d)
iconset_dir="$temp_dir/$basename.iconset"
mkdir -p "$iconset_dir"

# 生成各尺寸图片 - 使用正确的命名格式
sips -z 16 16 "$input" --out "$iconset_dir/icon_16x16.png"
sips -z 32 32 "$input" --out "$iconset_dir/icon_16x16@2x.png"
sips -z 32 32 "$input" --out "$iconset_dir/icon_32x32.png"
sips -z 64 64 "$input" --out "$iconset_dir/icon_32x32@2x.png"
sips -z 128 128 "$input" --out "$iconset_dir/icon_128x128.png"
sips -z 256 256 "$input" --out "$iconset_dir/icon_128x128@2x.png"
sips -z 256 256 "$input" --out "$iconset_dir/icon_256x256.png"
sips -z 512 512 "$input" --out "$iconset_dir/icon_256x256@2x.png"
sips -z 512 512 "$input" --out "$iconset_dir/icon_512x512.png"
sips -z 1024 1024 "$input" --out "$iconset_dir/icon_512x512@2x.png"

# 生成ICNS文件
iconutil -c icns "$iconset_dir" -o "$dest_dir/$basename.icns"

# 检查是否成功生成ICNS文件
if [ -f "$dest_dir/$basename.icns" ]; then
    echo "Successfully created ICNS file: $dest_dir/$basename.icns"
    
    # 清理临时目录
    rm -rf "$temp_dir"
    echo "Temporary files have been cleaned up."
else
    echo "Error: Failed to create ICNS file."
    echo "Temporary files are kept at: $temp_dir"
    exit 1
fi