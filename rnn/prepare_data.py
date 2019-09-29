import json
import re
from unidecode import unidecode

CMUDICT_FP = 'data/cmudict-0.7b.txt'
OUTPUT_CHAR_IDS_FP = 'data/cmudict-char-ids.json'
OUTPUT_PHONEME_IDS_FP = 'data/cmudict-phoneme-ids.json'
OUTPUT_CMUDICT_FP = 'data/cmudict-processed.json'
PRONUNCIATION_SUFFIX_REGEX = re.compile(r'\([0-9]+\)$')

char_ids_dict = {'SOS': 0, 'EOS': 1}  # char -> char_id
phoneme_ids_dict = {'SOS': 0, 'EOS': 1}  # phoneme -> phoneme_id
cmudict_pairs = []

with open(CMUDICT_FP, 'r', encoding='latin-1') as f:
    for line in f:
        if not line.startswith(';;;'):
            line_tokens = line.strip().split('  ')

            # word -> char_ids
            word = unidecode(line_tokens[0])
            word = re.sub(PRONUNCIATION_SUFFIX_REGEX, '', word)  # remove suffix denoting multiple pronunciations
            char_ids = []
            for char in word:
                if char not in char_ids_dict:
                    char_ids_dict[char] = len(char_ids_dict)
                char_ids.append(char_ids_dict[char])
            char_ids.append(char_ids_dict['EOS'])  # append EOS token

            # phonemes -> phoneme_ids
            phonemes = [unidecode(phoneme) for phoneme in line_tokens[1].split(' ')]
            phoneme_ids = []
            for phoneme in phonemes:
                if phoneme not in phoneme_ids_dict:
                    phoneme_ids_dict[phoneme] = len(phoneme_ids_dict)
                phoneme_ids.append(phoneme_ids_dict[phoneme])
            phoneme_ids.append(phoneme_ids_dict['EOS'])  # append EOS token

            cmudict_pairs.append((char_ids, phoneme_ids))

# save id dicts and pairs to file
with open(OUTPUT_CHAR_IDS_FP, 'w', encoding='ascii') as f:
    json.dump(char_ids_dict, f)

with open(OUTPUT_PHONEME_IDS_FP, 'w', encoding='ascii') as f:
    json.dump(phoneme_ids_dict, f)

with open(OUTPUT_CMUDICT_FP, 'w', encoding='ascii') as f:
    json.dump(cmudict_pairs, f)
