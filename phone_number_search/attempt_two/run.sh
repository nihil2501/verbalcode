#!/Users/orenmittman/.asdf/shims/ruby

CANDIDATES_FILE = File.join(File.dirname(__FILE__), "./candidates_2")
RESULTS_FILE = File.join(File.dirname(__FILE__), "./results_2")
NATURE_DIR = File.join(File.dirname(__FILE__), "./lexicon/nature")
TECHNOLOGY_DIR = File.join(File.dirname(__FILE__), "./lexicon/technology")
PHONE_NUMBER_SIZE = 10
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

      size_suffixes = Hash.new { |h, k| h[k] = [] }
      Dir.each_child(TECHNOLOGY_DIR) do |file|
        file = File.join(TECHNOLOGY_DIR, file)
        File.readlines(file, chomp: true).each do |word|
          size_suffixes[word.size] << word
        end
      end

      candidates.each do |prefix|
        suffixes = size_suffixes[PHONE_NUMBER_SIZE - prefix.size]
        suffixes.each do |suffix|
          candidate = prefix + suffix
          next unless search?(candidate)

          puts "FOUND!", candidate
          memo << candidate
        end
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

      Dir.each_child(NATURE_DIR) do |file|
        file = File.join(NATURE_DIR, file)
        File.readlines(file, chomp: true).each do |word|
          next unless search?(word)
          memo << word
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

get_results
