#!/Users/orenmittman/.asdf/shims/ruby

# Keypad layout image:
# https://en.wikipedia.org/wiki/Telephone_keypad#/media/File:Telephone-keypad2.svg

CODES_FILE = File.join(File.dirname(__FILE__), "../src/responder/exchange/codes.rs")
CODES_FILE_DELIMITER = "// SEARCH_SCRIPT_BOUNDARY"
CANDIDATES_FILE = File.join(File.dirname(__FILE__), "./candidates")
RESULTS_FILE = File.join(File.dirname(__FILE__), "./results")
PHONE_NUMBER_SIZE = 10
# Visually, shortest word was 3 letters.
MAX_START_WORD_SIZE = 7
SEARCH_PADDING = "*"

def get_results
  begin
    File.readlines(RESULTS_FILE, chomp: true).tap do |memo|
      puts RESULTS_FILE, "found"
    end
  rescue Errno::ENOENT
    [].tap do |memo|
      puts RESULTS_FILE, "not_found"
      puts "fetching candidates and building results"

      candidates = get_candidates
      puts "found #{candidates.size} candidates"

      candidates.each_with_index do |candidate, i|
        puts "#{i} of #{candidates.size}"
        next unless search?(candidate)
        memo << candidate
      end

      File.write(
        RESULTS_FILE,
        memo.join("\n")
      )
    end
  end
end

def get_candidates
  begin
    File.readlines(CANDIDATES_FILE, chomp: true).tap do |memo|
      puts CANDIDATES_FILE, "found"
    end
  rescue Errno::ENOENT
    [].tap do |memo|
      puts CANDIDATES_FILE, "not found"
      puts "checking start words and building candidates"

      file = File.read(CODES_FILE)
      data = file.split(CODES_FILE_DELIMITER)[1]
      words = eval(data)

      size_words = {}
      size_start_words = {}

      # Init hashes with increasing keys for more organized results.
      1.upto(PHONE_NUMBER_SIZE.pred) do |size|
        size_words[size + 1] = []
        size_start_words[size + 1] = []
      end

      words.each_with_index do |word, i|
        puts "#{i} of #{words.size}"
        
        next unless size_words.member?(word.size)
        size_words[word.size] << word

        next unless word.size <= MAX_START_WORD_SIZE
        # Much fewer matches for given start words.
        next unless search?(word)
        size_start_words[word.size] << word
      end

      size_start_words.each do |size, words|
        next if words.empty?

        complements = size_words[PHONE_NUMBER_SIZE - size]
        next if complements.empty?

        words.each do |word|
          complements.each do |complement|
            candidate = word + complement
            memo << candidate
          end
        end
      end
      
      File.write(
        CANDIDATES_FILE,
        memo.join("\n")
      )
    end
  end
end

def search?(candidate)
  puts "searching", candidate
  sleep 1

  candidate =
    candidate.upcase.ljust(
      PHONE_NUMBER_SIZE,
      SEARCH_PADDING
    )

  result = %x(
    twilio api:core:available-phone-numbers:local:list \
        --country-code US \
        --sms-enabled \
        --contains #{candidate}
  )

  !result.empty?
end

puts(get_results)
