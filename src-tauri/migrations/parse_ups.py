import os

path = './src-tauri/migrations'

files = sorted([os.path.join(path, f)
               for f in os.listdir(path) if '.up.' in f])

contents = [c.read() for c in [open(f) for f in files]]

with open(os.path.join(path, 'parsed_up.sql'), 'w') as f:
    f.write('\n\n'.join(contents))
