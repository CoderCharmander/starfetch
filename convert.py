"""
Converts constellations from the format in Haruno19/starfetch to the format
used in this Rust rewrite of the program.
"""
import sys
import json

with open(sys.argv[1], 'r') as f:
    orig_json = json.load(f)
stars = []
for i in range(1, 11):
    for k, v in orig_json["graph"][f"line{i}"].items():
        stars.append([int(k) - 1, i - 1, v])

with open(sys.argv[1], 'w') as f:
    json.dump({
        "title": orig_json["title"],
        "graph": stars,
        "name": orig_json["name"],
        "quadrant": orig_json["quadrant"],
        "right_ascension": orig_json["right ascension"],
        "declination": orig_json["declination"],
        "area": orig_json["area"],
        "main_stars": orig_json["main stars"]
    }, f)
