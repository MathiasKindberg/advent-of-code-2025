new day:
    cp -r days/aoc_ex days/aoc_{{day}}
    sed -i 's/name = "aoc_ex"/name = "aoc_{{day}}"/' days/aoc_{{day}}/Cargo.toml
