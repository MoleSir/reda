import os

header_files = []
for file_name in os.listdir('./third_party/si2-lef/clef/'):
    if file_name.endswith('.h'):
        header_files.append(file_name)
    
with open('./third_party/si2-lef/lef.h', 'w') as file:
    for header_file in header_files:
        file.write(f'#include "./clef/{header_file}"\n')

