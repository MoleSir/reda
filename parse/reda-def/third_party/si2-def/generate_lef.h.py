import os

header_files = []
for file_name in os.listdir('./third_party/si2-def/cdef/'):
    if file_name.endswith('.h'):
        header_files.append(file_name)
    
with open('./third_party/si2-def/def.h', 'w') as file:
    for header_file in header_files:
        file.write(f'#include "./cdef/{header_file}"\n')

