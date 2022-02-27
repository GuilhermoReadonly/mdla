"""
Use this script to generate a word list:

> python utils/sanitize_words.py 
"""

file_in = open("./utils/word_list_in")
file_out = open("./word_list", "w")

lines = file_in.readlines()

words = []
for (i, w) in enumerate(lines):
    w = w[1:-3]
    words.append(w)

file_out.write("\n".join(words))

file_out.close()
file_in.close()
