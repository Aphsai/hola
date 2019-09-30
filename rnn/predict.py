import argparse
import json
import torch
import torch.nn as nn
from torch.nn.functional import relu

# parse arguments
parser = argparse.ArgumentParser(prog='predict.py',
                                 description='Interface to predict phonemic pronunciation using encoder-decoder model.')
parser.add_argument('word', type=str)
parser.add_argument('--hidden_size', type=int, default=256)
args = parser.parse_args()

# constants
DEVICE = torch.device("cuda" if torch.cuda.is_available() else "cpu")
CHAR_IDS_FP = 'data/cmudict-char-ids.json'
PHONEME_IDS_FP = 'data/cmudict-phoneme-ids.json'
CMUDICT_FP = 'data/cmudict-processed.json'
ENCODER_MODEL_FP = 'model/encoder-{}.pt'.format(args.hidden_size)
DECODER_MODEL_FP = 'model/decoder-{}.pt'.format(args.hidden_size)

# TODO: move these definitions into separate file
class Encoder(nn.Module):
    def __init__(self, num_inputs, hidden_size):
        super(Encoder, self).__init__()

        self.hidden_size = hidden_size
        self.embedding = nn.Embedding(num_inputs, hidden_size)
        self.gru = nn.GRU(hidden_size, hidden_size)

    def forward(self, input, hidden):
        output = torch.reshape(self.embedding(input), (1, 1, -1))
        output, hidden = self.gru(output, hidden)
        return output, hidden

    def init_hidden(self):
        return torch.zeros(1, 1, self.hidden_size, device=DEVICE)

class Decoder(nn.Module):
    def __init__(self, num_outputs, hidden_size):
        super(Decoder, self).__init__()

        self.hidden_size = hidden_size
        self.embedding = nn.Embedding(num_outputs, hidden_size)
        self.gru = nn.GRU(hidden_size, hidden_size)
        self.out = nn.Linear(hidden_size, num_outputs)
        self.softmax = nn.LogSoftmax(dim=1)

    def forward(self, input, hidden):
        output = torch.reshape(self.embedding(input), (1, 1, -1))
        output = relu(output)
        output, hidden = self.gru(output, hidden)
        output = self.softmax(self.out(output[0]))
        return output, hidden

    def init_hidden(self):
        return torch.zeros(1, 1, self.hidden_size, device=DEVICE)

if __name__ == '__main__':
    input_char_ids = []
    output_phonemes = []

    # load id dicts from file
    with open(CHAR_IDS_FP, 'r', encoding='ascii') as f:
        char_ids_dict = json.load(f)

    with open(PHONEME_IDS_FP, 'r', encoding='ascii') as f:
        phoneme_ids_dict = json.load(f)

    with open(CMUDICT_FP, 'r', encoding='ascii') as f:
        cmudict_pairs = json.load(f)

    phoneme_ids_inv_dict = {phoneme_id: phoneme for phoneme, phoneme_id in phoneme_ids_dict.items()}

    max_length = max([max(len(pair[0]), len(pair[1])) for pair in cmudict_pairs])

    # load seq2seq model from file
    encoder = torch.load(ENCODER_MODEL_FP, map_location=DEVICE)
    decoder = torch.load(DECODER_MODEL_FP, map_location=DEVICE)
    encoder.eval()
    decoder.eval()

    # set up input_tensor
    for char in args.word:
        input_char_ids.append(char_ids_dict[char.upper()])

    input_tensor = torch.tensor(input_char_ids, device=DEVICE).view(-1, 1)
    input_length = input_tensor.size(0)

    # run input_tensor through model to get output_phonemes
    encoder_hidden = encoder.init_hidden()
    encoder_outputs = torch.zeros(max_length, encoder.hidden_size, device=DEVICE)

    for i in range(input_length):
        encoder_output, encoder_hidden = encoder(input_tensor[i], encoder_hidden)
        encoder_outputs[i] = encoder_output[0, 0]

    decoder_input = torch.tensor([[char_ids_dict['SOS']]], device=DEVICE)
    decoder_hidden = encoder_hidden

    for i in range(max_length):
        decoder_output, decoder_hidden = decoder(decoder_input, decoder_hidden)
        _, top_index = decoder_output.topk(1)
        decoder_input = top_index.squeeze().detach()
        if decoder_input.item() == char_ids_dict['EOS']:
            break
        else:
            output_phonemes.append(phoneme_ids_inv_dict[decoder_input.item()])

    print(' '.join(output_phonemes))
