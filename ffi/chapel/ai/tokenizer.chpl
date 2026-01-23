// Nuclear Chapel AI - Tokenizer
// Converts raw text/data into tokens for neural network training

module Tokenizer {
  use Map;
  
  // Token types
  enum TokenType {
    WORD,
    NUMBER,
    PUNCTUATION,
    SPECIAL,
    UNKNOWN
  }
  
  // Token record
  record Token {
    var id: int;
    var text: string;
    var type: TokenType;
    var embedding_index: int;
    var frequency: int;
  }
  
  // Tokenizer class
  class NuclearTokenizer {
    var vocab: map(string, int);
    var reverse_vocab: map(int, string);
    var vocab_size: int = 10000;
    var max_token_length: int = 32;
    var token_count: int = 0;
    
    // Special tokens
    var PAD_TOKEN = 0;
    var UNK_TOKEN = 1;
    var START_TOKEN = 2;
    var END_TOKEN = 3;
    var MASK_TOKEN = 4;
    
    proc init() {
      this.vocab = new map(string, int);
      this.reverse_vocab = new map(int, string);
      
      // Initialize special tokens
      vocab["<PAD>"] = PAD_TOKEN;
      vocab["<UNK>"] = UNK_TOKEN;
      vocab["<START>"] = START_TOKEN;
      vocab["<END>"] = END_TOKEN;
      vocab["<MASK>"] = MASK_TOKEN;
      
      reverse_vocab[PAD_TOKEN] = "<PAD>";
      reverse_vocab[UNK_TOKEN] = "<UNK>";
      reverse_vocab[START_TOKEN] = "<START>";
      reverse_vocab[END_TOKEN] = "<END>";
      reverse_vocab[MASK_TOKEN] = "<MASK>";
      
      token_count = 5;
    }
    
    // Add word to vocabulary
    proc addWord(word: string): int {
      if vocab.contains(word) {
        return vocab[word];
      }
      
      if token_count < vocab_size {
        var token_id = token_count;
        vocab[word] = token_id;
        reverse_vocab[token_id] = word;
        token_count += 1;
        return token_id;
      }
      
      return UNK_TOKEN;  // Unknown token
    }
    
    // Tokenize text to IDs
    proc tokenize(text: string): [] int {
      var words = text.split();
      var result: [1..words.size] int;
      
      for (i, word) in zip(1..words.size, words) {
        // Clean word
        var clean_word = word.toLower();
        
        // Add to vocab and get ID
        var token_id = addWord(clean_word);
        result[i] = token_id;
      }
      
      return result;
    }
    
    // Detokenize IDs back to text
    proc detokenize(token_ids: [] int): string {
      var result = "";
      
      for token_id in token_ids {
        if reverse_vocab.contains(token_id) {
          result += reverse_vocab[token_id] + " ";
        } else {
          result += "<UNK> ";
        }
      }
      
      return result.strip();
    }
    
    // Pad sequence to length
    proc padSequence(tokens: [] int, length: int): [] int {
      var padded: [1..length] int;
      
      // Fill with padding
      for i in 1..length {
        if i <= tokens.size {
          padded[i] = tokens[i];
        } else {
          padded[i] = PAD_TOKEN;
        }
      }
      
      return padded;
    }
    
    // Truncate sequence to length
    proc truncateSequence(tokens: [] int, length: int): [] int {
      if tokens.size <= length {
        return tokens;
      }
      
      var truncated: [1..length] int;
      for i in 1..length {
        truncated[i] = tokens[i];
      }
      return truncated;
    }
    
    // Encode dataset
    proc encodeDataset(texts: [] string, max_length: int = 512): [] [] int {
      var encoded: [1..texts.size, 1..max_length] int;
      
      for (i, text) in zip(1..texts.size, texts) {
        var tokens = tokenize(text);
        var processed = padSequence(tokens, max_length);
        encoded[i, ] = processed;
      }
      
      return encoded;
    }
    
    // Get vocabulary size
    proc getVocabSize(): int {
      return token_count;
    }
    
    // Save vocabulary
    proc saveVocab(filename: string) {
      use IO;
      var f = open(filename, iomode.cw);
      var writer = f.writer();
      
      for (word, id) in vocab.items() {
        writer.writeln(word + "\t" + id:string);
      }
      writer.close();
      f.close();
    }
    
    // Load vocabulary
    proc loadVocab(filename: string) {
      use IO;
      var f = open(filename, iomode.r);
      var reader = f.reader();
      
      var line: string;
      while reader.readLine(line) {
        var parts = line.split("\t");
        if parts.size >= 2 {
          var word = parts[0];
          var id = parts[1]:int;
          vocab[word] = id;
          reverse_vocab[id] = word;
        }
      }
      reader.close();
      f.close();
    }
  }
  
  // Batch tokenizer for parallel processing
  proc tokenizeBatch(texts: [] string, tokenizer: NuclearTokenizer): [] [] int {
    var batch_size = texts.size;
    var max_length = 512;
    
    var batch: [1..batch_size, 1..max_length] int;
    
    forall i in 1..batch_size {
      var tokens = tokenizer.tokenize(texts[i]);
      var padded = tokenizer.padSequence(tokens, max_length);
      batch[i, ] = padded;
    }
    
    return batch;
  }
  
  // Create embeddings from tokens
  proc createEmbeddings(tokens: [] int, embedding_dim: int = 32): [] [] real {
    var embeddings: [1..tokens.size, 1..embedding_dim] real;
    
    forall (i, token_id) in zip(1..tokens.size, tokens) {
      // Simple embedding: hash-based initialization
      var seed = (token_id * 73856093) ^ (i * 19349663);
      
      forall j in 1..embedding_dim {
        var rng_seed = seed + j:int;
        embeddings[i, j] = ((rng_seed * 2654435761) % 100000) / 100000.0;
      }
    }
    
    return embeddings;
  }
}
