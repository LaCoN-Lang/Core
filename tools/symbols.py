import os

def to_bytes(char):
    b = char.encode('utf-8')
    hex_str = ''.join(f'\\x{byte:02X}' for byte in b)
    return f'b"{hex_str}"'

def main():
    script_dir = os.path.dirname(os.path.abspath(__file__))

    input_path = os.path.join(script_dir, 'symbols.txt')
    output_path = os.path.join(script_dir, 'symbols-bytes.txt')

    with open(input_path, 'r', encoding='utf-8') as f_in, \
         open(output_path, 'w', encoding='utf-8') as f_out:

        for line in f_in:
            line = line.strip()
            if not line or line.startswith('//') or '=' not in line:
                continue

            name, char = line.split('=', 1)
            name = name.strip()
            char = char.strip()

            f_out.write(f'{to_bytes(char)} // {char}\n')

    print(f"File {output_path} successfully generated.")

if __name__ == '__main__':
    main()
