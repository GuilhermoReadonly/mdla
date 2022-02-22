"""
Use this script to generate a word list:

> python utils/sanitize_words.py 
"""

file_in = open("./utils/word_list_in")
file_out = open("./word_list", "w")

lines = file_in.readlines()

words = []
for (i, w) in enumerate(lines):
    if i%2==0:
        w = w.replace("\n", "")
        w = w.replace("un/une ", "")
        w = w.replace("une ", "")
        w = w.replace("un ", "")

        w = w.replace("à", "a")
        w = w.replace("â", "a")

        w = w.replace("é", "e")
        w = w.replace("è", "e")
        w = w.replace("ê", "e")

        w = w.replace("ï", "i")
        w = w.replace("î", "i")

        w = w.replace("û", "u")

        w = w.replace("ô", "o")
        w = w.replace("ö", "o")

        w = w.replace("ç", "c")
        w = w.replace("d'", "")
        w = w.replace("œ", "oe")
        w = w.strip()

        if len(w) > 4 and len(w) < 9 and "'un" not in w and "-" not in w:
            words.append(w)

file_out.write("\n".join(words))

file_out.close()
file_in.close()
