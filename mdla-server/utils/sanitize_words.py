"""
Use this script to generate a word list:

> python utils/sanitize_words.py 
"""


import unicodedata


def strip_accents(s: str) -> str:
    return "".join(
        c for c in unicodedata.normalize("NFD", s) if unicodedata.category(c) != "Mn"
    )


file_in = open("./mdla-server/utils/word_list_in")
file_out = open("./word_list_all.db", "w")

lines = file_in.readlines()

words = []
for (i, w) in enumerate(lines):

    if (
        ("α" in w)
        or ("β" in w)
        or ("γ" in w)
        or ("δ" in w)
        or ("ε" in w)
        or ("ζ" in w)
        or ("η" in w)
        or ("θ" in w)
        or ("κ" in w)
        or ("λ" in w)
        or ("μ" in w)
        or ("ξ" in w)
        or ("ο" in w)
        or ("π" in w)
        or ("ρ" in w)
        or ("σ" in w)
        or ("τ" in w)
        or ("υ" in w)
        or ("φ" in w)
        or ("χ" in w)
        or ("ψ" in w)
        or ("ω" in w)
        or ("ℓ" in w)
        or ("µ" in w)
        or ("ι" in w)
        or ("ν" in w)
    ):
        continue

    w_normalized = strip_accents(w.strip())
    w_normalized = w_normalized.replace("œ", "oe")
    w_normalized = w_normalized.replace("æ", "ae")

    w_normalized = w_normalized.upper()

    words.append(w_normalized)

words = list(set(words))

words.sort()

file_out.write("\n".join(words))

file_out.close()
file_in.close()
