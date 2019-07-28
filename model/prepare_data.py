import logging
import pandas as pd
from unidecode import unidecode

logging.basicConfig(format='%(levelname)s\t%(asctime)s\t%(message)s', level=logging.INFO)

CMUDICT_FP = 'data/cmudict-0.7b.txt'
OUTPUT_FP = 'data/cmudict-processed.csv'

char_ids_dict = {'SOS': 0, 'EOS': 1}  # char -> char_id
phoneme_ids_dict = {'SOS': 0, 'EOS': 1}  # phoneme -> phoneme_id
cmudict = {'word': [], 'char_ids': [], 'phonemes': [], 'phoneme_ids': []}

with open(CMUDICT_FP, 'r', encoding='latin-1') as f:
    for line in f:
        if not line.startswith(';;;'):
            line_tokens = line.strip().split('  ')

            # handle word
            word = unidecode(line_tokens[0])
            cmudict['word'].append(word)

            char_ids = []
            for char in word:
                if char not in char_ids_dict:
                    char_ids_dict[char] = len(char_ids_dict)
                char_ids.append(char_ids_dict[char])
            cmudict['char_ids'].append(char_ids)

            # handle phonemes
            phonemes = [unidecode(phoneme) for phoneme in line_tokens[1].split(' ')]
            cmudict['phonemes'].append(phonemes)

            phoneme_ids = []
            for phoneme in phonemes:
                if phoneme not in phoneme_ids_dict:
                    phoneme_ids_dict[phoneme] = len(phoneme_ids_dict)
                phoneme_ids.append(phoneme_ids_dict[phoneme])
            cmudict['phoneme_ids'].append(phoneme_ids)

cmudict_df = pd.DataFrame(cmudict)
cmudict_df.to_csv(OUTPUT_FP, index=False, encoding='ascii')
