import csv
import json


def csv_to_jsonl(csv_file, jsonl_file):
    with open(csv_file, "r", newline="") as csvfile:
        reader = csv.DictReader(csvfile)
        with open(jsonl_file, "w") as jsonlfile:
            for row in reader:
                json.dump(row, jsonlfile)
                jsonlfile.write("\n")


# Usage example:
csv_to_jsonl("combined_data.csv", "combined_data.jsonl")
