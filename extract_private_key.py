import json
import os

import base58

# 默认的 Solana 密钥对路径
keypair_path = os.path.expanduser("~/.config/solana/id.json")

# 读取 id.json 文件
with open(keypair_path, "r") as f:
    keypair_data = json.load(f)

private_key = keypair_data

# base58 编码私钥
private_key_base58 = base58.b58encode(bytes(private_key)).decode("utf-8")

# 打印结果
print("Base58 Encoded Private Key:", private_key_base58)
