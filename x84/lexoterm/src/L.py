def extract_text_between_markers(file_path):
    """
    Extracts and processes text between "L.start" and "L.stop" markers in a file.

    Args:
        file_path (str): The path to the input file.

    Returns:
        str: The extracted text between the markers, or None if not found.
    """

    with open(file_path, "r") as file:
        lines = file.readlines()

    start_index = None
    stop_index = None

    for i, line in enumerate(lines):
        if line.strip() == "L.start":
            start_index = i
        elif line.strip() == "L.stop":
            stop_index = i

        if start_index is not None and stop_index is not None:
            break  # Exit loop once both markers are found

    if start_index is None or stop_index is None:
        return None  # One or both markers missing

    relevant_lines = lines[start_index + 1 : stop_index]
    text_between_markers = "".join(relevant_lines)

    # Check if text matches the desired pattern
    if "func.cpu():" in text_between_markers and (
        "L <> i32" in text_between_markers or "L <> i64" in text_between_markers
    ):
        # Extract text within quotation marks
        start_quote = text_between_markers.find('"') + 1
        end_quote = text_between_markers.find('"', start_quote)
        extracted_text = text_between_markers[start_quote:end_quote]
        return extracted_text
    else:
        return None  # Text doesn't match the required structure
file_path = input("")
# Example usage
  # Replace with your actual file path
extracted_text = extract_text_between_markers(file_path)

if extracted_text:
    print(f"Extracted text: {extracted_text}")
else:
    print("Matching text not found or pattern doesn't match.")
