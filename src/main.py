import json
import numpy as np
import itertools as it
import os
from random import choice
from time import perf_counter

MISS = np.uint8(0)
MISPLACED = np.uint8(1)
EXACT = np.uint8(2)
DATA_DIR = os.path.join(os.path.dirname(os.path.realpath(__file__)), 'data')
PATTERN_MATRIX_FILE = os.path.join(DATA_DIR, 'pattern_matrix.npy')
WORD_LIST_FILE = os.path.join(DATA_DIR, 'words.json')
PATTERN_GRID_DATA = dict()
with open(WORD_LIST_FILE, 'r') as file:
    data = json.load(file)
    ANSWER_WORDS = data['answer_words']
    POSSIBLE_WORDS = data['possible_words'] + ANSWER_WORDS


def words_to_int_arrays(words):
    return np.array([[ord(c)for c in w] for w in words], dtype=np.uint8)


def generate_pattern_matrix(words1, words2):
    """
    A pattern for two words represents the wordle-similarity
    pattern (grey -> 0, yellow -> 1, green -> 2) but as an integer
    between 0 and 3^5. Reading this integer in ternary gives the
    associated pattern.
    This function computes the pairwise patterns between two lists
    of words, returning the result as a grid of hash values. Since
    this can be time-consuming, many operations that can be are vectorized
    (perhaps at the expense of easier readibility), and the result
    is saved to file so that this only needs to be evaluated once, and
    all remaining pattern matching is a lookup.
    """

    # Number of letters/words
    nl = len(words1[0])
    nw1 = len(words1)  # Number of words
    nw2 = len(words2)  # Number of words

    # Convert word lists to integer arrays
    word_arr1, word_arr2 = map(words_to_int_arrays, (words1, words2))

    # equality_grid keeps track of all equalities between all pairs
    # of letters in words. Specifically, equality_grid[a, b, i, j]
    # is true when words[i][a] == words[b][j]
    equality_grid = np.zeros((nw1, nw2, nl, nl), dtype=bool)
    for i, j in it.product(range(nl), range(nl)):
        equality_grid[:, :, i, j] = np.equal.outer(word_arr1[:, i], word_arr2[:, j])

    # full_pattern_matrix[a, b] should represent the 5-color pattern
    # for guess a and answer b, with 0 -> grey, 1 -> yellow, 2 -> green
    full_pattern_matrix = np.zeros((nw1, nw2, nl), dtype=np.uint8)

    # Green pass
    for i in range(nl):
        # matches[a, b] is true when words[a][i] = words[b][i]
        matches = equality_grid[:, :, i, i].flatten()
        full_pattern_matrix[:, :, i].flat[matches] = EXACT

        for k in range(nl):
            # If it's a match, mark all elements associated with
            # that letter, both from the guess and answer, as covered.
            # That way, it won't trigger the yellow pass.
            equality_grid[:, :, k, i].flat[matches] = False
            equality_grid[:, :, i, k].flat[matches] = False

    # Yellow pass
    for i, j in it.product(range(nl), range(nl)):
        matches = equality_grid[:, :, i, j].flatten()
        full_pattern_matrix[:, :, i].flat[matches] = MISPLACED
        for k in range(nl):
            # Similar to above, we want to mark this letter
            # as taken care of, both for answer and guess
            equality_grid[:, :, k, j].flat[matches] = False
            equality_grid[:, :, i, k].flat[matches] = False

    # Rather than representing a color pattern as a lists of integers,
    # store it as a single integer, whose ternary representations corresponds
    # to that list of integers.
    pattern_matrix = np.dot(
        full_pattern_matrix,
        (3**np.arange(nl)).astype(np.uint8)
    )

    return pattern_matrix


def check_matrix():
    if not PATTERN_GRID_DATA:
        if not os.path.exists(PATTERN_MATRIX_FILE):
            start_time = perf_counter()
            print("Generating pattern matrix")
            generate_full_pattern_matrix()
            print(f"Generated matrix in {round(perf_counter() - start_time, 2)} seconds")
        PATTERN_GRID_DATA['grid'] = np.load(PATTERN_MATRIX_FILE)
        PATTERN_GRID_DATA['words_to_index'] = dict(zip(
            POSSIBLE_WORDS, it.count()
        ))


def get_pattern_matrix(words1, words2):
    check_matrix()
    full_grid = PATTERN_GRID_DATA['grid']
    words_to_index = PATTERN_GRID_DATA['words_to_index']

    indices1 = [words_to_index[w] for w in words1]
    indices2 = [words_to_index[w] for w in words2]
    return full_grid[np.ix_(indices1, indices2)]


def get_pattern(guess, answer):
    if PATTERN_GRID_DATA:
        saved_words = PATTERN_GRID_DATA['words_to_index']
        print(saved_words)
        if guess in saved_words and answer in saved_words:
            return get_pattern_matrix([guess], [answer])[0, 0]
    return generate_pattern_matrix([guess], [answer])[0, 0]


def pattern_from_string(pattern_string):
    return sum((3**i) * int(c) for i, c in enumerate(pattern_string))


def pattern_to_int_list(pattern):
    result = []
    curr = pattern
    for _ in range(5):
        result.append(curr % 3)
        curr = curr // 3
    return result


def ternary_to_decimal(num):
    result = 0
    int_list = [int(i) for i in str(num)]
    for i, j in enumerate(int_list):
        result += 3 ** i * j
    return result


def pattern_to_string(pattern):
    d = {MISS: "⬛", MISPLACED: "🟨", EXACT: "🟩"}
    return "".join(d[x] for x in pattern_to_int_list(pattern))


def patterns_to_string(patterns):
    return "\n".join(map(pattern_to_string, patterns))


def generate_full_pattern_matrix():
    pattern_matrix = generate_pattern_matrix(POSSIBLE_WORDS, ANSWER_WORDS)
    # Save to file
    np.save(PATTERN_MATRIX_FILE, pattern_matrix)
    return pattern_matrix


def play_game():
    answer = choice(ANSWER_WORDS)
    patterns = []
    while True:
        guess = input("Guess: ")
        if len(guess) == 5:
            pattern = get_pattern(guess, answer)
            patterns.append(pattern)
            print(pattern)
            print(pattern_to_string(pattern))
            if pattern == 242 or len(patterns) == 6:
                break
        else:
            print("Guess must be 5 letters")
    print(f"{answer}\n{patterns_to_string(patterns)}")


def get_groups(guess, allowed_words=ANSWER_WORDS):
    check_matrix()
    full_grid = PATTERN_GRID_DATA['grid']
    words_to_index = PATTERN_GRID_DATA['words_to_index']

    index = [words_to_index[guess]]
    row = full_grid[index][0]
    groups = {}
    for j, k in enumerate(row):
        word = ANSWER_WORDS[j]
        if word in allowed_words:
            if k in groups:
                groups[k].append(word)
            else:
                groups[k] = [word]
    return groups


def print_groups(groups, guess):
    group_length = [len(i) for i in groups.values()]
    print(f"\nGroups for {guess}")
    print(f"Total number of groups: {len(groups)}")
    print(f"Average group length: {round(sum(group_length) / len(groups), 2)}")
    print(f"Longest group: {max(group_length)}")
    for j in groups.items():
        print(f"{pattern_to_string(j[0])} {len(j[1])}: {j[1][:14]}")


def specific_groups():
    allowed = ANSWER_WORDS
    while True:
        guess = input("Guess: ")
        groups = get_groups(guess, allowed)
        print_groups(groups, guess)
        try:
            clue = ternary_to_decimal(input("Clue (ternary): "))
            if clue:
                allowed = groups[clue]
        except ValueError:
            print("Invalid input")
        except KeyError:
            print(f"no possible input with pattern {pattern_to_string(clue)}")
        print(allowed[:100])
        if len(allowed) == 1:
            break


if __name__ == '__main__':
    specific_groups()
